/*
 * Copyright 2024 William Swartzendruber
 *
 * This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a
 * copy of the MPL was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 *
 * SPDX-License-Identifier: MPL-2.0
 */

#[cfg(test)]
mod tests;

use std::{
    fs::File,
    io::{stdin, BufReader, ErrorKind, Read, Result},
};
use dsp::tf::pq_eotf;
use byteorder::{LittleEndian, ReadBytesExt};
use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};

fn main() {

    let matches = app_from_crate!()
        .arg(Arg::with_name("width")
            .long("width")
            .short("w")
            .value_name("PIXELS")
            .help("Width of the input video stream")
            .takes_value(true)
            .required(true)
            .validator(|value| {
                if value.parse::<usize>().is_ok() {
                    Ok(())
                } else {
                    Err("Must be an unsigned integer".to_string())
                }
            })
        )
        .arg(Arg::with_name("height")
            .long("height")
            .short("h")
            .value_name("PIXELS")
            .help("Height of the input video stream")
            .takes_value(true)
            .required(true)
            .validator(|value| {
                if value.parse::<usize>().is_ok() {
                    Ok(())
                } else {
                    Err("Must be an unsigned integer".to_string())
                }
            })
        )
        .arg(Arg::with_name("input")
            .index(1)
            .value_name("INPUT-FILE")
            .help("Raw input video stream (PQ, BT.2020, RGB48LE); use - for STDIN")
            .required(true)
        )
        .after_help(format!("Copyright © 2024 William Swartzendruber\n\
            Licensed under the Mozilla Public License 2.0\n\
            <{}>", env!("CARGO_PKG_REPOSITORY")).as_str())
        .get_matches();
    let width = matches.value_of("width").unwrap().parse::<usize>().unwrap();
    let height = matches.value_of("height").unwrap().parse::<usize>().unwrap();
    let input_value = matches.value_of("input").unwrap();
    let (mut stdin_read, mut file_read);
    let mut input = BufReader::<&mut dyn Read>::new(
        if input_value == "-" {
            stdin_read = stdin();
            &mut stdin_read
        } else {
            file_read = File::open(input_value)
                .expect("Could not open input file for writing.");
            &mut file_read
        }
    );
    let mut max_channel = 0_u16;

    'frames: loop {

        match read_frame_max_channel(&mut input, width * height) {
            Ok(frame_max_channel) => {
                max_channel = max_channel.max(frame_max_channel);
            }
            Err(err) => {
                match err.kind() {
                    ErrorKind::UnexpectedEof => break 'frames,
                    _ => panic!("Could not read frame from input stream: {:?}", err),
                }
            }
        }
    }

    println!("MaxCLL: {}", to_nits(max_channel));
}

fn read_frame_max_channel(input: &mut dyn Read, count: usize) -> Result<u16> {

    let mut max_channel = 0_u16;

    for _ in 0..(3 * count) {
        max_channel = max_channel.max(input.read_u16::<LittleEndian>()?);
    }

    Ok(max_channel)
}

fn to_nits(max_channel: u16) -> u16 {
    (pq_eotf(max_channel as f64 / 65_535.0) * 10_000.0).ceil() as u16
}
