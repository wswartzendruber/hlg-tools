/*
 * Copyright © 2021 William Swartzendruber
 * Licensed under the Open Software License version 3.0
 */

use super::tf::{pq_eotf, pq_oetf};

pub struct ToneMapper {
    lwp: f64,
    ml: f64,
    ks: f64,
}

impl ToneMapper {

    pub fn new(peak: f64) -> Self {

        let lwp = pq_oetf(peak);
        let ml = pq_oetf(0.10) / lwp;
        let ks = 1.5 * ml - 0.5;

        Self { lwp, ml, ks }
    }

    pub fn map(&self, e: f64) -> f64 {

        let e1 = pq_oetf(e) / self.lwp;
        let e2 =
            if e1 < self.ks {
                e1
            } else {
                p(self.ks, self.ml, e1)
            };

        pq_eotf(e2 * self.lwp).min(0.1)
    }
}

fn p(ks: f64, ml: f64, b: f64) -> f64 {

    let t = (b - ks) / (1.0 - ks);
    let t2 = t.powf(2.0);
    let t3 = t.powf(3.0);

    (2.0 * t3 - 3.0 * t2 + 1.0) * ks
    +
    (t3 - 2.0 * t2 + t) * (1.0 - ks)
    +
    (-2.0 * t3 + 3.0 * t2) * ml
}
