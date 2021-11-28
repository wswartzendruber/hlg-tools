/*
 * SPDX-FileCopyrightText: 2021 William Swartzendruber <wswartzendruber@gmail.com>
 *
 * SPDX-License-Identifier: OSL-3.0
 */

#[cfg(test)]
mod tests;

use super::pixel::RgbPixel;

const G: f64 = 2.4;

//
// BT.1886
//

pub struct Bt1886 {
    pub a: f64,
    pub b: f64,
}

impl Bt1886 {

    pub fn new(lw: f64, lb: f64) -> Self {
        Self {
            a: (lw.powf(1.0 / G) - lb.powf(1.0 / G)).powf(G),
            b: lb.powf(1.0 / G) / (lw.powf(1.0 / G) - lb.powf(1.0 / G)),
        }
    }

    pub fn eotf(&self, v: f64) -> f64 {
        self.a * (v + self.b).max(0.0).powf(G)
    }

    pub fn ieotf(&self, l: f64) -> f64 {
        (l / self.a).powf(1.0 / G) - self.b
    }
}

pub fn pq_e_to_dl(e: f64) -> f64 {

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

pub fn pq_dl_to_e(e: f64) -> f64 {

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

pub fn hlg_sl_to_e(o: f64) -> f64 {

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

pub fn hlg_dl_to_sl(pixel: RgbPixel) -> RgbPixel {

    //
    // ITU-R BT.2100-2
    // Page 8
    // Note 5i
    //

    pixel * pixel.to_yxy().y.powf(-0.16666666666666663).min(f64::MAX)
}
