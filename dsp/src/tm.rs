/*
 * Copyright © 2021 William Swartzendruber
 * Licensed under the Open Software License version 3.0
 */

#[cfg(test)]
mod tests;

use super::tf::{pq_e_to_dl, pq_dl_to_e};

pub struct ToneMapper {
    target: f64,
    lwp: f64,
    ml: f64,
    ks: f64,
}

impl ToneMapper {

    pub fn new(peak: f64, target: f64) -> Self {

        let lwp = pq_dl_to_e(peak);
        let ml = pq_dl_to_e(target) / lwp;
        let ks = 1.5 * ml - 0.5;

        Self { target, lwp, ml, ks }
    }

    pub fn map(&self, o: f64) -> f64 {
        pq_e_to_dl(self.eetf(pq_dl_to_e(o))).min(self.target)
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
