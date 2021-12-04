/*
 * SPDX-FileCopyrightText: 2021 William Swartzendruber <wswartzendruber@gmail.com>
 *
 * SPDX-License-Identifier: OSL-3.0
 */

use std::{
    fs::File,
    io::{stdout, BufWriter, Write},
};
use dsp::{PqHlgMapper, pixel::RgbPixel};
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
        .arg(Arg::with_name("lum-scale")
            .long("lum-scale")
            .short("l")
            .value_name("FACTOR")
            .help("Scales the linear brightness of the input video by the specified factor")
            .takes_value(true)
            .required(false)
            .default_value("1")
            .validator(|value| {
                let lum_scale = value.parse::<f64>();
                if lum_scale.is_err() {
                    return Err("Must be a floating point value".to_string())
                }
                let lum_scale_value = lum_scale.unwrap();
                if !lum_scale_value.is_normal() {
                    return Err("Must be a normal number".to_string())
                }
                if !lum_scale_value.is_sign_positive() {
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
            PQ-to-HLG conversion LUT. If --lum-scale is provided, the linear input brightness \
            will be scaled by the provided factor while performing gamma correction. This will \
            cause the --max-cll value to be internally adjusted as well. If the internal \
            MaxCLL value then exceeds 1,000 nits, BT.2408 luminosity tone mapping will be \
            applied to compress the input to 1,000 nits. From there, the signal will be \
            converted to HLG. The generated LUTs are completely full range with 0.0 \
            representing minimum brightness and 1.0 representing maximum brightness.\n\n\
            Copyright © 2021 William Swartzendruber\n\
            Licensed under the Open Software License version 3.0\n\
            <{}>", env!("CARGO_PKG_REPOSITORY")).as_str())
        .get_matches();
    let title = matches.value_of("title");
    let lum_scale = matches.value_of("lum-scale").unwrap().parse::<f64>().unwrap();
    let max_cll = matches.value_of("max-cll").unwrap().parse::<f64>().unwrap();
    let mapper = PqHlgMapper::new(max_cll, lum_scale);
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

    if title.is_some() {
        writeln!(output, "TITLE \"{}\"", title.unwrap()).unwrap();
    }

    writeln!(output, "LUT_3D_SIZE {}", size).unwrap();

    for b in 0..size {
        for g in 0..size {
            for r in 0..size {

                let pixel = mapper.map(RgbPixel {
                    red: r as f64 / (size - 1) as f64,
                    green: g as f64 / (size - 1) as f64,
                    blue: b as f64 / (size - 1) as f64,
                });

                writeln!(output, "{} {} {}",
                    pixel.red.min(1.0) as f32,
                    pixel.green.min(1.0) as f32,
                    pixel.blue.min(1.0) as f32,
                ).unwrap();
            }
        }
    }
}
