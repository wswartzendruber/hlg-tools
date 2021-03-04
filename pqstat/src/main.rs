/*
 * Copyright © 2021 William Swartzendruber
 * Licensed under the Open Software License version 3.0
 */

#[cfg(test)]
mod tests;

mod io;

use std::{
    fs::File,
    io::{stdin, BufReader, ErrorKind, Read},
};
use io::read_frame;
use tf::{pq_eotf, Pixel};
use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};

struct FrameStats {
    max_cll: f64,
    max_channel: f64,
}

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
        .after_help(format!("Copyright © 2021 William Swartzendruber\n\
            Licensed under the Open Software License version 3.0\n\
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
    let mut max_cll = 0.0_f64;
    let mut max_channel = 0.0_f64;
    let mut frame = vec![Pixel { red: 0.0, green: 0.0, blue: 0.0 }; width * height];

    'frames: loop {

        if let Err(err) = read_frame(&mut input, &mut frame) {
            match err.kind() {
                ErrorKind::UnexpectedEof => break 'frames,
                _ => panic!("Could not read frame from input stream: {:?}", err),
            }
        }

        let stats = frame_stats(&frame);

        max_cll = max_cll.max(stats.max_cll);
        max_channel = max_channel.max(stats.max_channel);
    }

    println!("MaxCLL....: {}", max_cll * 10_000.0);
    println!("MaxChannel: {}", pq_eotf(max_channel));
}

fn frame_stats(frame: &[Pixel]) -> FrameStats {

    let mut max_cll = 0.0_f64;
    let mut max_channel = 0.0_f64;

    for pixel in frame.iter() {
        max_cll = max_cll.max(
            0.2627 * pq_eotf(pixel.red)
            + 0.6780 * pq_eotf(pixel.green)
            + 0.0593 * pq_eotf(pixel.blue)
        );
        max_channel = max_channel.max(pixel.red);
        max_channel = max_channel.max(pixel.green);
        max_channel = max_channel.max(pixel.blue);
    }

    FrameStats { max_cll, max_channel }
}
