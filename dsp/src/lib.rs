/*
 * Copyright 2023 William Swartzendruber
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
use tf::{hlg_eotf, hlg_iootf, hlg_oetf, pq_eotf, pq_ieotf, sdr_o_to_e};
use tm::{sdn_tone_map, Bt2408ToneMapper, ToneMapMethod};

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
}

impl PqHlgMapper {

    pub fn new(max_cll: f64, tm_method: ToneMapMethod) -> Self {
        Self::new_by_factor(1.0, max_cll, tm_method)
    }

    pub fn new_by_ref_white(
        ref_white: f64,
        max_cll: f64,
        tm_method: ToneMapMethod,
    ) -> Self {

        let factor = scale_nits_factor(ref_white, 203.0);

        Self::new_by_factor(factor, max_cll, tm_method)
    }

    pub fn new_by_factor(
        factor: f64,
        max_cll: f64,
        tm_method: ToneMapMethod,
    ) -> Self {
        Self {
            prepper: PqPrepper::new(factor, max_cll, tm_method),
        }
    }

    pub fn map(&self, input: RgbPixel) -> RgbPixel {

        let mut pixel = self.prepper.map(input);

        // PQ DISPLAY LINEAR -> HLG DISPLAY LINEAR
        pixel *= 10.0;

        // HLG DISPLAY LINEAR -> HLG SCENE LINEAR
        pixel = hlg_iootf(pixel);

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

        let factor = scale_nits_factor(ref_white, 203.0);

        Self::new_by_factor(factor, max_cll, tm_method)
    }

    pub fn new_by_factor(factor: f64, max_cll: f64, tm_method: ToneMapMethod) -> Self {
        Self { prepper: PqPrepper::new(factor, max_cll, tm_method) }
    }

    pub fn map(&self, input: RgbPixel) -> RgbPixel {

        let pixel = self.prepper.map(input);
        let mut y = pixel
            .bt2020_to_xyz()
            .to_oklab()
            .monochrome()
            .to_xyz()
            .to_rgb_bt709()
            .y_bt709();

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

        let peak = (RgbPixel::new_y(max_cll / 10_000.0).bt2020_to_xyz().to_oklab() * factor)
            .to_xyz().to_rgb_bt2020().y_bt2020();
        let tm = Bt2408ToneMapper::new(peak, tm_method);

        Self { factor, tm }
    }

    fn map(&self, input: RgbPixel) -> RgbPixel {

        let mut pixel = input;

        // PQ SIGNAL -> DISPLAY LINEAR
        pixel = pixel.with_each_channel(|x| pq_eotf(x)).clamp(0.0, 1.0);

        // REFERENCE WHITE ADJUSTMENT
        pixel = (pixel.bt2020_to_xyz().to_oklab() * self.factor).to_xyz().to_rgb_bt2020();

        // TONE MAPPING
        pixel = self.tm.map(pixel);

        // 1,000 NIT CLAMPING
        pixel.clamp(0.0, 0.1)
    }
}

//
// Shared
//

fn scale_nits_factor(from: f64, to: f64) -> f64 {

    let l_from = RgbPixel::new_y(from / 10_000.0).bt2020_to_xyz().to_oklab().l;
    let l_to = RgbPixel::new_y(to / 10_000.0).bt2020_to_xyz().to_oklab().l;

    l_to / l_from
}
