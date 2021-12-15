/*
 * SPDX-FileCopyrightText: 2021 William Swartzendruber <wswartzendruber@gmail.com>
 *
 * SPDX-License-Identifier: OSL-3.0
 */

use dsp::tf::{Bt1886, pq_e_to_dl};
use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};

fn main() {

    let matches = app_from_crate!()
        .arg(Arg::with_name("sdr-level")
            .index(1)
            .value_name("RATIO")
            .help("Observed signal level in the SDR stream")
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
        .arg(Arg::with_name("pq-level")
            .index(2)
            .value_name("RATIO")
            .help("Observed signal level in the PQ stream")
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
    let sdr_e = matches.value_of("sdr-level").unwrap().parse::<f64>().unwrap();
    let pq_e = matches.value_of("pq-level").unwrap().parse::<f64>().unwrap();
    let sdr_o = Bt1886::new(0.0, 253.75).eotf(sdr_e);
    let pq_o = pq_e_to_dl(pq_e) * 10_000.0;

    println!("{}", sdr_o / pq_o);
}
