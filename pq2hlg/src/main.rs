/*
 * Copyright 2022 William Swartzendruber
 *
 * This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a
 * copy of the MPL was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 *
 * SPDX-License-Identifier: MPL-2.0
 */

use std::{
    fs::File,
    io::{stdout, BufWriter, Write},
};
use dsp::{
    Mapper,
    PqHlgMapper,
    PqSdrMapper,
    pixel::Pixel,
    tm::ToneMapMethod,
};
use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};

fn main() {

    let matches = app_from_crate!()
        .arg(Arg::with_name("title")
            .long("title")
            .short("t")
            .value_name("STRING")
            .help("Title of the LUT")
            .takes_value(true)
            .required(false)
            .validator(|value| {
                if value.contains("\"") {
                    return Err("Must not contain a double quote mark".to_string())
                }
                if value.len() > 242 {
                    return Err("Must not have a length greater than 242 bytes".to_string())
                }
                Ok(())
            })
        )
        .arg(Arg::with_name("preview")
            .long("preview")
            .short("p")
            .help("Generates a black and white SDR preview LUT instead of a HLG one")
            .takes_value(false)
        )
        .arg(Arg::with_name("lum-scale")
            .long("lum-scale")
            .short("l")
            .value_name("FACTOR")
            .help("Scales the linear brightness of the input video by the specified factor")
            .takes_value(true)
            .required(false)
            .conflicts_with("ref-white")
            .validator(|value| {
                let ref_white = value.parse::<f64>();
                if ref_white.is_err() {
                    return Err("Must be a floating point value".to_string())
                }
                let ref_white_value = ref_white.unwrap();
                if !ref_white_value.is_normal() {
                    return Err("Must be a normal number".to_string())
                }
                if !ref_white_value.is_sign_positive() {
                    return Err("Must be a positive number".to_string())
                }
                Ok(())
            })
        )
        .arg(Arg::with_name("ref-white")
            .long("ref-white")
            .short("r")
            .value_name("NITS")
            .help("Brightness of the input video stream's reference white level")
            .takes_value(true)
            .required(false)
            .validator(|value| {
                let ref_white = value.parse::<f64>();
                if ref_white.is_err() {
                    return Err("Must be a floating point value".to_string())
                }
                let ref_white_value = ref_white.unwrap();
                if !ref_white_value.is_normal() {
                    return Err("Must be a normal number".to_string())
                }
                if !ref_white_value.is_sign_positive() {
                    return Err("Must be a positive number".to_string())
                }
                Ok(())
            })
        )
        .arg(Arg::with_name("max-cll")
            .long("max-cll")
            .short("m")
            .value_name("NITS")
            .help("MaxCLL value of the input.")
            .takes_value(true)
            .required(false)
            .default_value("1000")
            .validator(|value| {
                let max_cll = value.parse::<f64>();
                if max_cll.is_err() {
                    return Err("Must be a floating point value".to_string())
                }
                let max_cll_value = max_cll.unwrap();
                if !max_cll_value.is_normal() {
                    return Err("Must be a normal number".to_string())
                }
                if !max_cll_value.is_sign_positive() {
                    return Err("Must be a positive number".to_string())
                }
                if max_cll_value > 10_000.0 {
                    return Err("Must not exceed 10,000.0.".to_string())
                }
                Ok(())
            })
        )
        .arg(Arg::with_name("tone-map-method")
            .long("tone-map-method")
            .help("Tone mapping method to use.")
            .takes_value(true)
            .required(false)
            .possible_values(&["rgb", "maxrgb"])
            .default_value("maxrgb")
        )
        .arg(Arg::with_name("size")
            .long("size")
            .short("s")
            .value_name("COUNT")
            .help("The size of each dimension of the 3D LUT")
            .takes_value(true)
            .required(false)
            .default_value("64")
            .validator(|value| {
                let size = value.parse::<usize>();
                if size.is_err() {
                    return Err("Must be an unsigned integer value".to_string())
                }
                let size_value = size.unwrap();
                if size_value < 2 || size_value > 256 {
                    return Err("Must be between 2 and 256".to_string())
                }
                Ok(())
            })
        )
        .arg(Arg::with_name("output")
            .index(1)
            .value_name("OUTPUT-FILE")
            .help("Output Cube LUT file; use - for STDOUT")
            .required(true)
        )
        .after_help(format!("This utility follows the BT.2408 method for generating a \
            PQ-to-HLG conversion LUT. If either --lum-scale or --ref-white are provided, \
            the linear input brightness will either be scaled by the provided factor, or \
            scaled to bring the provided reference white level to 203 nits, respectively. This \
            will cause the --max-cll value to be internally adjusted as well. If the internal \
            MaxCLL value then exceeds 1,000 nits, BT.2408 tone mapping will be applied to \
            compress the input to 1,000 nits using either the maxRGB or R'G'B' method. From \
            there, the signal will be converted to HLG. The generated LUTs are completely full \
            range with 0.0 representing minimum brightness and 1.0 representing maximum \
            brightness.\n\n\
            Optionally, a preview LUT can be generated to convert the input to black and white \
            SDR. This can be used to compare the converted output to available BT.709 frames \
            once they are also converted to black and white. In this way, --lum-scale can be \
            adjusted until the two sets of screenshots match as much as possible.\n\n\
            Copyright © 2022 William Swartzendruber\n\
            Licensed under the Mozilla Public License 2.0\n\
            <{}>", env!("CARGO_PKG_REPOSITORY")).as_str())
        .get_matches();
    let mut header = vec![];
    let title = matches.value_of("title");
    let max_cll = matches.value_of("max-cll").unwrap().parse::<f64>().unwrap();
    let tm_method = match matches.value_of("tone-map-method").unwrap() {
        "rgb" => ToneMapMethod::Rgb,
        "maxrgb" => ToneMapMethod::MaxRgb,
        _ => unreachable!("--tone-map-method select is irrational"),
    };
    let mapper: Box<dyn Mapper> = if matches.is_present("preview") {
        header.push(String::from("preview: true"));
        Box::new(
            match (matches.value_of("lum-scale"), matches.value_of("ref-white")) {
                (None, None) => {
                    PqSdrMapper::new(max_cll, tm_method)
                }
                (Some(lum_scale), None) => {
                    header.push(format!("lum-scale: {}", lum_scale));
                    PqSdrMapper::new_by_factor(lum_scale.parse::<f64>().unwrap(), max_cll, tm_method)
                }
                (None, Some(ref_white)) => {
                    header.push(format!("ref-white: {}", ref_white));
                    PqSdrMapper::new_by_ref_white(ref_white.parse::<f64>().unwrap(), max_cll, tm_method)
                }
                (Some(_), Some(_)) => {
                    unreachable!("--lum-scale and --ref-white were somehow both defined")
                }
            }
        )
    } else {
        header.push(String::from("preview: false"));
        Box::new(
            match (matches.value_of("lum-scale"), matches.value_of("ref-white")) {
                (None, None) => {
                    PqHlgMapper::new(max_cll, tm_method)
                }
                (Some(lum_scale), None) => {
                    header.push(format!("lum-scale: {}", lum_scale));
                    PqHlgMapper::new_by_factor(lum_scale.parse::<f64>().unwrap(), max_cll, tm_method)
                }
                (None, Some(ref_white)) => {
                    header.push(format!("ref-white: {}", ref_white));
                    PqHlgMapper::new_by_ref_white(ref_white.parse::<f64>().unwrap(), max_cll, tm_method)
                }
                (Some(_), Some(_)) => {
                    unreachable!("--lum-scale and --ref-white were somehow both defined")
                }
            }
        )
    };
    let size = matches.value_of("size").unwrap().parse::<usize>().unwrap();
    let output_value = matches.value_of("output").unwrap();
    let (mut stdout_write, mut file_write);
    let mut output = BufWriter::<&mut dyn Write>::new(
        if output_value == "-" {
            stdout_write = stdout();
            &mut stdout_write
        } else {
            file_write = File::create(output_value)
                .expect("Could not open output file for writing.");
            &mut file_write
        }
    );

    writeln!(output, "# Generated by PQ2HLG {}", env!("CARGO_PKG_VERSION")).unwrap();
    writeln!(output, "# max-cll: {}", max_cll).unwrap();
    for line in header.iter() {
        writeln!(output, "# {}", line).unwrap();
    }
    if title.is_some() {
        writeln!(output, "TITLE \"{}\"", title.unwrap()).unwrap();
    }
    writeln!(output, "LUT_3D_SIZE {}", size).unwrap();

    for b in 0..size {
        for g in 0..size {
            for r in 0..size {

                let pixel = mapper.map(Pixel {
                    red: r as f64 / (size - 1) as f64,
                    green: g as f64 / (size - 1) as f64,
                    blue: b as f64 / (size - 1) as f64,
                }).clamp(0.0, 1.0);

                writeln!(output, "{} {} {}",
                    pixel.red as f32,
                    pixel.green as f32,
                    pixel.blue as f32,
                ).unwrap();
            }
        }
    }
}
