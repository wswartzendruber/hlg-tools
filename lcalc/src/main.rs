/*
 * SPDX-FileCopyrightText: 2021 William Swartzendruber <wswartzendruber@gmail.com>
 *
 * SPDX-License-Identifier: OSL-3.0
 */

use dsp::tf::pq_e_to_dl;
use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};

fn main() {

    let matches = app_from_crate!()
        .arg(Arg::with_name("level")
            .index(1)
            .value_name("RATIO")
            .help("Observed signal level of reference white in the PQ stream")
            .required(true)
            .validator(|value| {
                let lum_scale = value.parse::<f64>();
                if lum_scale.is_err() {
                    return Err("Must be a floating point value".to_string())
                }
                let lum_scale_value = lum_scale.unwrap();
                if lum_scale_value < 0.0 || 1.0 < lum_scale_value {
                    return Err("Must be between 0.0 and 1.0".to_string())
                }
                Ok(())
            })
        )
        .after_help(format!("Copyright © 2021 William Swartzendruber\n\
            Licensed under the Open Software License version 3.0\n\
            <{}>", env!("CARGO_PKG_REPOSITORY")).as_str())
        .get_matches();
    let level = matches.value_of("level").unwrap().parse::<f64>().unwrap();

    println!("{}", 0.0203 / pq_e_to_dl(level));
}
