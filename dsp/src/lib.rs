/*
 * SPDX-FileCopyrightText: 2021 William Swartzendruber <wswartzendruber@gmail.com>
 *
 * SPDX-License-Identifier: OSL-3.0
 */

#[cfg(test)]
mod tests;

pub mod pixel;
pub mod tf;
pub mod tm;

use pixel::RgbPixel;
use tf::{hlg_sl_to_e, pq_e_to_dl, hlg_dl_to_sl, sdr_o_to_e};
use tm::{bt2446_c_tone_map, Bt2408ToneMapper};

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

    pub fn new(max_cll: f64, factor: f64) -> Self {
        Self { prepper: PqPrepper::new(max_cll, factor) }
    }

    pub fn map(&self, input: RgbPixel) -> RgbPixel {

        let mut pixel = self.prepper.prep(input);

        // PQ DISPLAY LINEAR -> HLG SCENE LINEAR
        pixel = hlg_dl_to_sl(pixel);

        // SCENE LINEAR -> HLG SIGNAL
        RgbPixel {
            red: hlg_sl_to_e(pixel.red).min(1.0),
            green: hlg_sl_to_e(pixel.green).min(1.0),
            blue: hlg_sl_to_e(pixel.blue).min(1.0),
        }
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

    pub fn new(max_cll: f64, factor: f64) -> Self {
        Self { prepper: PqPrepper::new(max_cll, factor) }
    }

    pub fn map(&self, input: RgbPixel) -> RgbPixel {

        let mut pixel = self.prepper.prep(input);

        // TONE MAPPING TO SDR
        pixel = bt2446_c_tone_map(pixel * 10.0);

        // MONOCHROME
        let y = pixel.to_yxy().y;
        pixel = RgbPixel {
            red: y,
            green: y,
            blue: y,
        };

        // SDR LINEAR -> SDR GAMMA
        RgbPixel {
            red: sdr_o_to_e(pixel.red).min(1.0),
            green: sdr_o_to_e(pixel.green).min(1.0),
            blue: sdr_o_to_e(pixel.blue).min(1.0),
        }
    }
}

impl Mapper for PqSdrMapper {

    fn map(&self, input: RgbPixel) -> RgbPixel {
        self.map(input)
    }
}

//
// PQ Prepper
//

struct PqPrepper {
    factor: f64,
    gamma: f64,
    peak: f64,
    tone_mapper: Bt2408ToneMapper,
}

impl PqPrepper {

    fn new(max_cll: f64, factor: f64) -> Self {

        let gamma = 1.111_f64.powf(factor.log2());
        let peak = (max_cll / 10_000.0 * factor).powf(1.0 / gamma);
        let tone_mapper = Bt2408ToneMapper::new(peak);

        Self { factor, gamma, peak, tone_mapper }
    }

    fn prep(&self, input: RgbPixel) -> RgbPixel {

        let mut rgb_pixel = input;

        // PQ SIGNAL -> DISPLAY LINEAR
        rgb_pixel = RgbPixel {
            red: pq_e_to_dl(rgb_pixel.red),
            green: pq_e_to_dl(rgb_pixel.green),
            blue: pq_e_to_dl(rgb_pixel.blue),
        };

        // SCALING
        rgb_pixel = (rgb_pixel.to_yxy() * self.factor).powf(1.0 / self.gamma).to_rgb();

        // TONE MAPPING
        if self.peak > 0.1 {
            rgb_pixel.red = self.tone_mapper.map(rgb_pixel.red);
            rgb_pixel.green = self.tone_mapper.map(rgb_pixel.green);
            rgb_pixel.blue = self.tone_mapper.map(rgb_pixel.blue);
        }

        rgb_pixel
    }
}
