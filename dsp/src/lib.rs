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

pub mod pixel;
pub mod tf;
pub mod tm;

use pixel::Pixel;
use tf::{hlg_sl_to_e, pq_e_to_dl, hlg_dl_to_sl, sdr_o_to_e};
use tm::{sdn_tone_map, Bt2408ToneMapper};

//
// Mapper
//

pub trait Mapper {
    fn map(&self, input: Pixel) -> Pixel;
}

//
// PQ -> HLG Mapper
//

pub struct PqHlgMapper {
    factor: f64,
    peak: f64,
    tone_mapper: Bt2408ToneMapper,
}

impl PqHlgMapper {

    pub fn new(max_cll: f64) -> Self {
        Self::new_by_factor(1.0, max_cll)
    }

    pub fn new_by_ref_white(ref_white: f64, max_cll: f64) -> Self {
        Self::new_by_factor(203.0 / ref_white, max_cll)
    }

    pub fn new_by_factor(factor: f64, max_cll: f64) -> Self {

        let peak = max_cll / 10_000.0 * factor;
        let tone_mapper = Bt2408ToneMapper::new(peak);

        Self { factor, peak, tone_mapper }
    }

    pub fn map(&self, input: Pixel) -> Pixel {

        let mut pixel = input;

        // PQ SIGNAL -> DISPLAY LINEAR
        pixel = pixel.with_each_channel(|x| pq_e_to_dl(x)).clamp();

        // REFERENCE WHITE ADJUSTMENT
        pixel *= self.factor;

        // TONE MAPPING
        if self.peak > 0.1 {
            pixel = pixel.with_each_channel(|x| self.tone_mapper.map(x));
        }

        // PQ DISPLAY LINEAR -> HLG DISPLAY LINEAR
        pixel = (pixel * 10.0).clamp();

        // HLG DISPLAY LINEAR -> HLG SCENE LINEAR
        pixel = hlg_dl_to_sl(pixel).clamp();

        // SCENE LINEAR -> HLG SIGNAL
        pixel.with_each_channel(|x| hlg_sl_to_e(x)).clamp()
    }
}

impl Mapper for PqHlgMapper {

    fn map(&self, input: Pixel) -> Pixel {
        self.map(input)
    }
}

//
// PQ -> SDR Preview Mapper
//

pub struct PqSdrMapper {
    factor: f64,
    peak: f64,
    tone_mapper: Bt2408ToneMapper,
}

impl PqSdrMapper {

    pub fn new(max_cll: f64) -> Self {
        Self::new_by_factor(1.0, max_cll)
    }

    pub fn new_by_ref_white(ref_white: f64, max_cll: f64) -> Self {
        Self::new_by_factor(203.0 / ref_white, max_cll)
    }

    pub fn new_by_factor(factor: f64, max_cll: f64) -> Self {

        let peak = max_cll / 10_000.0 * factor;
        let tone_mapper = Bt2408ToneMapper::new(peak);

        Self { factor, peak, tone_mapper }
    }

    pub fn map(&self, input: Pixel) -> Pixel {

        let mut pixel = input;

        // PQ SIGNAL -> DISPLAY LINEAR
        pixel = pixel.with_each_channel(|x| pq_e_to_dl(x)).clamp();

        // REFERENCE WHITE ADJUSTMENT
        pixel *= self.factor;

        // TONE MAPPING
        if self.peak > 0.1 {
            pixel = pixel.with_each_channel(|x| self.tone_mapper.map(x));
        }

        // MONOCHROME
        let mut y = pixel.y().clamp(0.0, 0.1);

        // TONE MAPPING (FROM 1,000 NITS TO 100 NITS)
        y = sdn_tone_map(y * 10.0);

        // SDR LINEAR -> SDR GAMMA
        Pixel {
            red: sdr_o_to_e(y),
            green: sdr_o_to_e(y),
            blue: sdr_o_to_e(y),
        }.clamp()
    }
}

impl Mapper for PqSdrMapper {

    fn map(&self, input: Pixel) -> Pixel {
        self.map(input)
    }
}
