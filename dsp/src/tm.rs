/*
 * SPDX-FileCopyrightText: 2021 William Swartzendruber <wswartzendruber@gmail.com>
 *
 * SPDX-License-Identifier: OSL-3.0
 */

#[cfg(test)]
mod tests;

use super::{
    tf::{pq_e_to_dl, pq_dl_to_e},
};

pub struct Bt2408ToneMapper {
    peak: f64,
    lwp: f64,
    ml: f64,
    ks: f64,
}

impl Bt2408ToneMapper {

    pub fn new(peak: f64) -> Self {

        let lwp = pq_dl_to_e(peak);
        let ml = pq_dl_to_e(0.10) / lwp;
        let ks = 1.5 * ml - 0.5;

        Self { peak, lwp, ml, ks }
    }

    pub fn map(&self, o: f64) -> f64 {
        if o < self.peak {
            pq_e_to_dl(self.eetf(pq_dl_to_e(o)))
        } else {
            0.1
        }
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
