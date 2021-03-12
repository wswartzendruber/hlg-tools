/*
 * Copyright © 2021 William Swartzendruber
 * Licensed under the Open Software License version 3.0
 */

#[cfg(test)]
mod tests;

use super::Pixel;

pub fn pq_eotf(e: f64) -> f64 {

    //
    // ITU-R BT.2100-2
    // Table 4
    //

    (
        (e.powf(0.012683313515655966) - 0.8359375).max(0.0)
        /
        (18.8515625 - 18.6875 * e.powf(0.012683313515655966))
    )
    .powf(6.277394636015326)
}

pub fn pq_oetf(e: f64) -> f64 {

    //
    // ITU-R BT.2100-2
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
    // Table 5
    //

    if o < 0.08333333333333333 {
        (3.0 * o).sqrt()
    } else {
        0.17883277 * (12.0 * o - 0.28466892).ln() + 0.559910729529562
    }
}

pub fn pq_hlg_iootf(pixel: Pixel) -> Pixel {

    //
    // The BBC R&D Method of PQ to HLG Transcoding
    //

    let np = pixel * 10.0;
    let yg = np.y().powf(-0.166666667).min(f64::MAX);

    np * yg
}
