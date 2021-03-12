/*
 * Copyright © 2021 William Swartzendruber
 * Licensed under the Open Software License version 3.0
 */

use super::Pixel;

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

    let rd = pixel.red * 10.0;
    let gd = pixel.green * 10.0;
    let bd = pixel.blue * 10.0;
    let yg = (0.2627 * rd + 0.6780 * gd + 0.0593 * bd).powf(-0.166666667).min(f64::MAX);
    let red = rd * yg;
    let green = gd * yg;
    let blue = bd * yg;

    Pixel { red, green, blue }
}
