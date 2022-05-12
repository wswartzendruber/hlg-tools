/*
 * This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a
 * copy of the MPL was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 *
 * Copyright 2022 William Swartzendruber
 *
 * SPDX-License-Identifier: MPL-2.0
 */

#[cfg(test)]
mod tests;

use super::{
    Pixel,
    tf::{pq_e_to_dl, pq_dl_to_e},
};

pub enum ToneMapMethod {
    Rgb,
    MaxRgb,
}

pub struct Bt2408ToneMapper {
    peak: f64,
    lwp: f64,
    ml: f64,
    ks: f64,
    method: ToneMapMethod,
}

impl Bt2408ToneMapper {

    pub fn new(peak: f64, method: ToneMapMethod) -> Self {

        let lwp = pq_dl_to_e(peak);
        let ml = pq_dl_to_e(0.10) / lwp;
        let ks = 1.5 * ml - 0.5;

        Self { peak, lwp, ml, ks, method }
    }

    pub fn map(&self, pixel: Pixel) -> Pixel {
        if self.peak > 0.1 {
            match self.method {
                ToneMapMethod::Rgb => {
                    self.map_rgb(pixel)
                }
                ToneMapMethod::MaxRgb => {
                    self.map_max_rgb(pixel)
                }
            }
        } else {
            pixel
        }
    }

    fn map_rgb(&self, pixel: Pixel) -> Pixel {
        pixel.with_each_channel(|x| pq_e_to_dl(self.eetf(pq_dl_to_e(x))))
    }

    fn map_max_rgb(&self, pixel: Pixel) -> Pixel {

        let m1 = pixel.red.max(pixel.green.max(pixel.blue));

        if m1 > 0.0 {
            let m2 = pq_e_to_dl(self.eetf(pq_dl_to_e(m1)));
            let factor = m2 / m1;
            pixel.with_each_channel(|x| (factor * x))
        } else {
            pixel
        }
    }

    fn eetf(&self, e: f64) -> f64 {

        let e1 = e / self.lwp;
        let e2 =
            /*
             * The boundary provided in BT.2408-4 is incorrect. If used, it will cause `e2` to
             * approach infinity as the luminosity scaling factor approaches 1.0 from the
             * positive side. I stumbled upon the solution to this in a GitHub issue:
             * (https://github.com/mpv-player/mpv/issues/7984). The work here seems to have been
             * done by an individual named Florian Hoech. The specific answer to the infinity
             * problem lies in a comment inside code that appears to be under a GPL-3.0 license.
             * I am unsure if an idea mentioned in a comment is covered by GPL-3.0. However, the
             * author is welcome to contact me with grievances if he or she feels that I have
             * unfairly utilized their labor here.
             */
            if self.ks < e1 && e1 <= 1.0 {
                self.p(e1)
            } else {
                e1
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
