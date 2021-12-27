/*
 * This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a
 * copy of the MPL was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 *
 * Copyright 2021 William Swartzendruber
 *
 * SPDX-License-Identifier: MPL-2.0
 */

#[cfg(test)]
mod tests;

use super::tf::{pq_e_to_dl, pq_dl_to_e};

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

pub fn sdn_tone_map(o: f64) -> f64 {

    //
    // SDN = Stupid Desmos Naivety
    //
    // I just sat in front of Desmos trying random equations until I got a curve that looked the
    // way I wanted it to. Basically, HLG has reference white at 203 nits and max white at 1,000
    // nits. SDR has reference white at 80 nits and max white at 100 nits. So 0-203 nits should
    // linearly map to 0-80 nits with a sharp knee for 203-1,000 nits mapping to 80-100 nits.
    //

    if o < 0.0 {
        0.0
    } else if 0.0 <= o && o <= 0.203 {
        o / 0.2537
    } else if 0.203 < o && o <= 1.0 {
        (o - 0.19).ln() / 21.0 + 1.007
    } else {
        1.0
    }
}
