/*
 * Copyright 2022 William Swartzendruber
 *
 * This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a
 * copy of the MPL was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 *
 * SPDX-License-Identifier: MPL-2.0
 */

#[cfg(test)]
mod tests;

pub mod pixel;
pub mod tf;
pub mod tm;

use pixel::RgbPixel;
use tf::{hlg_compensate, hlg_eotf, hlg_iootf, hlg_oetf, pq_eotf, pq_ieotf, sdr_o_to_e};
use tm::{sdn_tone_map, Bt2408ToneMapper, ToneMapMethod};

pub const RED_FACTOR: f64 = 0.2627;
pub const GREEN_FACTOR: f64 = 0.6780;
pub const BLUE_FACTOR: f64 = 0.0593;

//
// Mapper
//

pub trait Mapper {
    fn map(&self, input: RgbPixel) -> RgbPixel;
}

//
// PQ -> HLG Mapper
//

pub struct PqHlgMapper {
    prepper: PqPrepper,
    compensate: bool,
}

impl PqHlgMapper {

    pub fn new(max_cll: f64, tm_method: ToneMapMethod, compensate: bool) -> Self {
        Self::new_by_factor(1.0, max_cll, tm_method, compensate)
    }

    pub fn new_by_ref_white(
        ref_white: f64,
        max_cll: f64,
        tm_method: ToneMapMethod,
        compensate: bool
    ) -> Self {

        let l_from = RgbPixel::new_y(ref_white / 10_000.0).to_xyz().to_oklab().l;
        let l_to = RgbPixel::new_y(203.0 / 10_000.0).to_xyz().to_oklab().l;

        Self::new_by_factor(l_to / l_from, max_cll, tm_method, compensate)
    }

    pub fn new_by_factor(
        factor: f64,
        max_cll: f64,
        tm_method: ToneMapMethod,
        compensate: bool
    ) -> Self {
        Self {
            prepper: PqPrepper::new(factor, max_cll, tm_method),
            compensate,
        }
    }

    pub fn map(&self, input: RgbPixel) -> RgbPixel {

        let mut pixel = self.prepper.map(input);

        // PQ DISPLAY LINEAR -> HLG DISPLAY LINEAR
        pixel *= 10.0;

        // HLG DISPLAY LINEAR -> HLG SCENE LINEAR
        pixel = hlg_iootf(pixel);

        if self.compensate == true {
            // HLG COMPENSATION
            pixel = hlg_compensate(pixel);
        }

        // SCENE LINEAR -> HLG SIGNAL
        pixel.with_each_channel(|x| hlg_oetf(x))
    }
}

impl Mapper for PqHlgMapper {

    fn map(&self, input: RgbPixel) -> RgbPixel {
        self.map(input)
    }
}

//
// PQ -> SDR Preview Mapper
//

pub struct PqSdrMapper {
    prepper: PqPrepper,
}

impl PqSdrMapper {

    pub fn new(max_cll: f64, tm_method: ToneMapMethod) -> Self {
        Self::new_by_factor(1.0, max_cll, tm_method)
    }

    pub fn new_by_ref_white(ref_white: f64, max_cll: f64, tm_method: ToneMapMethod) -> Self {
        Self::new_by_factor(203.0 / ref_white, max_cll, tm_method)
    }

    pub fn new_by_factor(factor: f64, max_cll: f64, tm_method: ToneMapMethod) -> Self {
        Self { prepper: PqPrepper::new(factor, max_cll, tm_method) }
    }

    pub fn map(&self, input: RgbPixel) -> RgbPixel {

        let pixel = self.prepper.map(input);

        // MONOCHROME
        let mut y = pixel.y();

        // TONE MAPPING (FROM 1,000 NITS TO 100 NITS)
        y = sdn_tone_map(y * 10.0);

        // SDR LINEAR -> SDR GAMMA
        RgbPixel {
            red: sdr_o_to_e(y),
            green: sdr_o_to_e(y),
            blue: sdr_o_to_e(y),
        }
    }
}

impl Mapper for PqSdrMapper {

    fn map(&self, input: RgbPixel) -> RgbPixel {
        self.map(input)
    }
}

//
// HLG to PQ Mapper
//

pub struct HlgPqMapper {
    max_cll: f64,
    gamma: f64,
}

impl HlgPqMapper {

    pub fn new(max_cll: f64) -> Self {

        let gamma = 1.2 + 0.42 * (max_cll / 1_000.0).log10();

        Self {
            max_cll,
            gamma,
        }
    }

    pub fn map(&self, input: RgbPixel) -> RgbPixel {

        let mut pixel = input;

        // HLG SIGNAL -> HLG DISPLAY LINEAR
        pixel = hlg_eotf(pixel, self.gamma);

        // HLG DISPLAY LINEAR -> PQ DISPLAY LINEAR
        pixel *= self.max_cll / 10_000.0;

        // PQ DISPLAY LINEAR -> PQ SIGNAL
        pixel.with_each_channel(|x| pq_ieotf(x))
    }
}

impl Mapper for HlgPqMapper {

    fn map(&self, input: RgbPixel) -> RgbPixel {
        self.map(input)
    }
}

//
// PQ Prepper
//

struct PqPrepper {
    factor: f64,
    tm: Bt2408ToneMapper,
}

impl PqPrepper {

    fn new(factor: f64, max_cll: f64, tm_method: ToneMapMethod) -> Self {

        let peak = max_cll / 10_000.0 * factor;
        let tm = Bt2408ToneMapper::new(peak, tm_method);

        Self { factor, tm }
    }

    fn map(&self, input: RgbPixel) -> RgbPixel {

        let mut pixel = input;

        // PQ SIGNAL -> DISPLAY LINEAR
        pixel = pixel.with_each_channel(|x| pq_eotf(x)).clamp(0.0, 1.0);

        // REFERENCE WHITE ADJUSTMENT
        pixel = (pixel.to_xyz().to_oklab() * self.factor).to_xyz().to_rgb();

        // TONE MAPPING
        pixel = self.tm.map(pixel);

        // 1,000 NIT CLAMPING
        pixel.clamp(0.0, 0.1)
    }
}
