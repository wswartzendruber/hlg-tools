/*
 * SPDX-FileCopyrightText: 2021 William Swartzendruber <wswartzendruber@gmail.com>
 *
 * SPDX-License-Identifier: OSL-3.0
 */

#[cfg(test)]
mod tests;

use super::{
    pixel::RgbPixel,
    tf::{pq_e_to_dl, pq_dl_to_e},
};

pub struct Bt2408ToneMapper {
    lwp: f64,
    ml: f64,
    ks: f64,
}

impl Bt2408ToneMapper {

    pub fn new(peak: f64) -> Self {

        let lwp = pq_dl_to_e(peak);
        let ml = pq_dl_to_e(0.10) / lwp;
        let ks = 1.5 * ml - 0.5;

        Self { lwp, ml, ks }
    }

    pub fn map(&self, o: f64) -> f64 {
        pq_e_to_dl(self.eetf(pq_dl_to_e(o.clamp(0.0, 1.0)))).clamp(0.0, 0.1)
    }

    fn eetf(&self, e: f64) -> f64 {

        let e1 = e / self.lwp;
        let e2 =
            if e1 < self.ks {
                e1
            } else {
                self.p(e1)
            };

        e2 * self.lwp
    }

    fn p(&self, b: f64) -> f64 {

        let t = (b - self.ks) / (1.0 - self.ks);
        let t2 = t.powf(2.0);
        let t3 = t.powf(3.0);

        (2.0 * t3 - 3.0 * t2 + 1.0) * self.ks
        +
        (t3 - 2.0 * t2 + t) * (1.0 - self.ks)
        +
        (-2.0 * t3 + 3.0 * t2) * self.ml
    }
}

pub fn bt2446_c_tone_map(o: RgbPixel) -> RgbPixel {

    //
    // ITU-R BT.2446-0 Section 5 (Method C)
    //
    // Map from HLG to SDR.
    //

    // PRE-SCALING
    let mut rgb = o * 1_000.0;

    // CONSTANTS
    let a0 = 0.165;
    let a1 = 1.0 - 2.0 * a0;
    let a2 = -a0;
    let a3 = 1.0 - a0;
    let ac = 1.0 / (1.0 - 3.0 * a0);
    let k1 = 0.83802;
    let k2 = 15.09968;
    let k3 = 0.74204;
    let k4 = 78.99439;
    let y_hdr_ip = 58.5 / k1;

    // 5.1.2 CROSSTALK MATRIX
    rgb = RgbPixel {
        red: a1 * rgb.red + a0 * rgb.green + a0 * rgb.blue,
        green: a0 * rgb.red + a1 * rgb.green + a0 * rgb.blue,
        blue: a0 * rgb.red + a0 * rgb.green + a1 * rgb.blue,
    };

    // 5.1.3 CONVERSION TO YXY
    let mut yxy = rgb.to_yxy();

    // 5.1.4 TONE MAPPING
    yxy.y = if yxy.y < y_hdr_ip {
        k1 * yxy.y
    } else {
        k2 * (yxy.y / y_hdr_ip - k3).ln() + k4
    };

    // 5.1.5 CONVERSION TO RGB LINEAR SIGNAL
    rgb = yxy.to_rgb();

    // 5.1.6 INVERSE CROSSTALK MATRIX
    rgb = RgbPixel {
        red: ac * (a3 * rgb.red + a2 * rgb.green + a2 * rgb.blue),
        green: ac * (a2 * rgb.red + a3 * rgb.green + a2 * rgb.blue),
        blue: ac * (a2 * rgb.red + a2 * rgb.green + a3 * rgb.blue),
    };

    // POST-SCALING
    rgb * 0.01
}
