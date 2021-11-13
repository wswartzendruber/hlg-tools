/*
 * SPDX-FileCopyrightText: 2021 William Swartzendruber <wswartzendruber@gmail.com>
 *
 * SPDX-License-Identifier: OSL-3.0
 */

#[cfg(test)]
mod tests;

use super::{
    Pixel,
    tf::{pq_e_to_dl, pq_dl_to_e},
};

pub struct Bt2390ToneMapper {
    lwp: f64,
    ml: f64,
    ks: f64,
}

impl Bt2390ToneMapper {

    pub fn new(peak: f64) -> Self {

        let lwp = pq_dl_to_e(peak);
        let ml = pq_dl_to_e(0.10) / lwp;
        let ks = 1.5 * ml - 0.5;

        Self { lwp, ml, ks }
    }

    pub fn map(&self, o: f64) -> f64 {
        pq_e_to_dl(self.eetf(pq_dl_to_e(o))).min(0.1)
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

/// Maps from a HLG linear light [Pixel] to a BT.2020 SDR linear light [Pixel] in accordance
/// with ITU-R BT.2446-0 Section 5.
///
/// * `o` - The HLG linear light [Pixel] to be tone mapped, ranging from 0 to 1.
pub fn bt2446_c_tone_map(o: Pixel) -> Pixel {

    //
    // ITU-R BT.2446-0 Section 5
    //
    // Map from HLG to SDR.
    //

    // CONSTANTS
    let a0 = 0.165;
    let a1 = 1.0 - 2.0 * a0;
    let a2 = 1.0 - a0;
    let ac = 1.0 / (1.0 - 3.0 * a0);
    let k1 = 0.83802;
    let k2 = 15.09968;
    let k3 = 0.74204;
    let k4 = 78.99439;
    let y_hdr_ip = 58.5 / k1;

    // 5.1.2 CROSSTALK MATRIX
    let r_x_hdr = a1 * o.red + a0 * o.green + a0 * o.blue;
    let g_x_hdr = a0 * o.red + a1 * o.green + a0 * o.blue;
    let b_x_hdr = a0 * o.red + a0 * o.green + a1 * o.blue;

    // 5.1.3 CONVERSION TO YXY
    let x_hdr = 0.6370 * r_x_hdr + 0.1446 * g_x_hdr + 0.1689 * b_x_hdr;
    let y_hdr = 0.2627 * r_x_hdr + 0.6780 * g_x_hdr + 0.0593 * b_x_hdr;
    let z_hdr = 0.0000 * r_x_hdr + 0.0281 * g_x_hdr + 1.0610 * b_x_hdr;
    let x = x_hdr / (x_hdr + y_hdr + z_hdr);
    let y = y_hdr / (x_hdr + y_hdr + z_hdr);

    // 5.1.4 TONE MAPPING
    let y_sdr = if y_hdr < y_hdr_ip {
        k1 * y_hdr
    } else {
        k2 * (y_hdr / y_hdr_ip - k3).ln() + k4
    };

    // 5.1.5 CONVERSION TO RGB LINEAR SIGNAL
    let x_sdr = (x / y) * y_sdr;
    let z_sdr = ((1.0 - x - y) / y) * y_sdr;
    let r_x_sdr = 1.7167 * x_sdr - 0.3557 * y_sdr - 0.2534 * z_sdr;
    let g_x_sdr = -0.6667 * x_sdr + 1.6165 * y_sdr + 0.0158 * z_sdr;
    let b_x_sdr = 0.0176 * x_sdr - 0.0428 * y_sdr + 0.9421 * z_sdr;

    // 5.1.6 INVERSE CROSSTALK MATRIX
    Pixel {
        red: (a2 * r_x_sdr - a0 * g_x_sdr - a0 * b_x_sdr) * ac,
        green: (-a0 * r_x_sdr + a2 * g_x_sdr - a0 * b_x_sdr) * ac,
        blue: (-a0 * r_x_sdr - a0 * g_x_sdr + a2 * b_x_sdr) * ac,
    }
}
