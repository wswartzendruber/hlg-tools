/*
 * Copyright 2023 William Swartzendruber
 *
 * This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a
 * copy of the MPL was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 *
 * SPDX-License-Identifier: MPL-2.0
 */

#[cfg(test)]
mod tests;

use super::RgbPixel;

pub fn pq_eotf(e: f64) -> f64 {

    //
    // ITU-R BT.2100-2
    // Page 5
    // Table 4
    //

    (
        (e.powf(0.012683313515655966) - 0.8359375).max(0.0)
        /
        (18.8515625 - 18.6875 * e.powf(0.012683313515655966))
    )
    .powf(6.277394636015326)
}

pub fn pq_ieotf(e: f64) -> f64 {

    //
    // ITU-R BT.2100-2
    // Page 6
    // Table 4
    //

    (
        (0.8359375 + 18.8515625 * e.powf(0.1593017578125))
        /
        (1.0 + 18.6875 * e.powf(0.1593017578125))
    )
    .powf(78.84375)
}

pub fn hlg_eotf(pixel: RgbPixel, gamma: f64) -> RgbPixel {

    //
    // ITU-R BT.2100-2
    // Page 7
    // Table 5
    //

    hlg_ootf(pixel.with_each_channel(|x| hlg_ioetf(x)), gamma)
}

pub fn hlg_oetf(o: f64) -> f64 {

    //
    // ITU-R BT.2100-2
    // Page 6
    // Table 5
    //

    if o < 0.08333333333333333 {
        (3.0 * o).sqrt()
    } else {
        0.17883277 * (12.0 * o - 0.28466892).ln() + 0.559910729529562
    }
}

pub fn hlg_ioetf(e: f64) -> f64 {

    //
    // ITU-R BT.2100-2
    // Page 7
    // Table 5
    //

    let a = 0.17883277_f64;
    let b = 1.0 - 4.0 * a;
    let c = 0.5 - a * (4.0 * a).ln();

    if e <= 0.5 {
        e.powf(2.0) / 3.0
    } else {
        (((e - c) / a).exp() + b) / 12.0
    }
}

pub fn hlg_ootf(pixel: RgbPixel, gamma: f64) -> RgbPixel {

    //
    // ITU-R BT.2100-2
    // Page 6
    // Table 5
    //

    let y = pixel.y_bt2020().powf(gamma - 1.0);

    pixel.with_each_channel(|x| y * x)
}

pub fn hlg_iootf(pixel: RgbPixel) -> RgbPixel {

    //
    // ITU-R BT.2100-2
    // Page 8
    // Note 5i
    //

    pixel * pixel.y_bt2020().powf(-0.16666666666666663).min(f64::MAX)
}

pub fn sdr_e_to_o(o: f64) -> f64 {
    o.powf(2.4).clamp(0.0, 1.0)
}

pub fn sdr_o_to_e(o: f64) -> f64 {
    o.powf(0.4166666666666667).clamp(0.0, 1.0)
}
