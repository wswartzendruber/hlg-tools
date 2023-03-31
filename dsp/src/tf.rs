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

use super::{RED_FACTOR, GREEN_FACTOR, BLUE_FACTOR, Pixel};

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

pub fn hlg_iootf(pixel: Pixel) -> Pixel {

    //
    // ITU-R BT.2100-2
    // Page 8
    // Note 5i
    //

    pixel * pixel.y().powf(-0.16666666666666663).min(f64::MAX)
}

pub fn hlg_compensate(pixel: Pixel) -> Pixel {

    //
    // ITU-R BT.2408-4
    // Page 25
    // Section 6.5
    //

    let mut working = pixel;

    while working.red > 1.0 || working.green > 1.0 || working.blue > 1.0 {

        if working.red > 1.0 && working.green < 1.0 && working.blue == 1.0 {

            // R->G

            let excess = working.red - 1.0;
            let nits = excess * RED_FACTOR;
            let addition = nits / GREEN_FACTOR;

            working.red -= excess;
            working.green += addition;

        } else if working.red > 1.0 && working.green == 1.0 && working.blue < 1.0 {

            // R->B

            let excess = working.red - 1.0;
            let nits = excess * RED_FACTOR;
            let addition = nits / BLUE_FACTOR;

            working.red -= excess;
            working.blue += addition;

        } else if working.red < 1.0 && working.green > 1.0 && working.blue == 1.0 {

            // G->R

            let excess = working.green - 1.0;
            let nits = excess * GREEN_FACTOR;
            let addition = nits / RED_FACTOR;

            working.red += addition;
            working.green -= excess;

        } else if working.red == 1.0 && working.green > 1.0 && working.blue < 1.0 {

            // G->B

            let excess = working.green - 1.0;
            let nits = excess * GREEN_FACTOR;
            let addition = nits / BLUE_FACTOR;

            working.green -= excess;
            working.blue += addition;

        } else if working.red < 1.0 && working.green == 1.0 && working.blue > 1.0 {

            // B->R

            let excess = working.blue - 1.0;
            let nits = excess * BLUE_FACTOR;
            let addition = nits / RED_FACTOR;

            working.red += addition;
            working.blue -= excess;

        } else if working.red == 1.0 && working.green < 1.0 && working.blue > 1.0 {

            // B->G

            let excess = working.blue - 1.0;
            let nits = excess * BLUE_FACTOR;
            let addition = nits / GREEN_FACTOR;

            working.blue -= excess;
            working.green += addition;

        } else if working.red < 1.0 && working.green < 1.0 && working.blue > 1.0 {

            // B->RG

            let excess = working.blue - 1.0;
            let nits = excess * BLUE_FACTOR;
            let factor = (RED_FACTOR + GREEN_FACTOR) / 2.0;
            let addition = nits / 2.0 / factor;

            working.red += addition;
            working.green += addition;
            working.blue -= excess;

        } else if working.red < 1.0 && working.green > 1.0 && working.blue < 1.0 {

            // G->RB

            let excess = working.green - 1.0;
            let nits = excess * GREEN_FACTOR;
            let factor = (RED_FACTOR + BLUE_FACTOR) / 2.0;
            let addition = nits / 2.0 / factor;

            working.red += addition;
            working.green -= excess;
            working.blue += addition;

        } else if working.red > 1.0 && working.green < 1.0 && working.blue < 1.0 {

            // R->GB

            let excess = working.red - 1.0;
            let nits = excess * RED_FACTOR;
            let factor = (GREEN_FACTOR + BLUE_FACTOR) / 2.0;
            let addition = nits / 2.0 / factor;

            working.red -= excess;
            working.green += addition;
            working.blue += addition;

        } else if working.red < 1.0 && working.green > 1.0 && working.blue > 1.0 {

            // GB->R

            let excess = working.green.min(working.blue) - 1.0;
            let nits = excess * GREEN_FACTOR + excess * BLUE_FACTOR;

            working.red += nits / RED_FACTOR;
            working.green -= excess;
            working.blue -= excess;

        } else if working.red > 1.0 && working.green < 1.0 && working.blue > 1.0 {

            // RB->G

            let excess = working.red.min(working.blue) - 1.0;
            let nits = excess * RED_FACTOR + excess * BLUE_FACTOR;

            working.red -= excess;
            working.green += nits / GREEN_FACTOR;
            working.blue -= excess;

        } else if working.red > 1.0 && working.green > 1.0 && working.blue < 1.0 {

            // RG->B

            let excess = working.red.min(working.green) - 1.0;
            let nits = excess * RED_FACTOR + excess * GREEN_FACTOR;

            working.red -= excess;
            working.green -= excess;
            working.blue += nits / BLUE_FACTOR;

        } else {
            unreachable!("HLG compensator has an invalid state: {:?}", working)
        }
    }

    return working;
}

pub fn sdr_o_to_e(o: f64) -> f64 {
    o.powf(0.4166666666666667).clamp(0.0, 1.0)
}
