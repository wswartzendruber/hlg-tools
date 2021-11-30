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
use tf::{hlg_sl_to_e, pq_e_to_dl, hlg_dl_to_sl, Bt1886};
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

        // HLG DISPLAY LINEAR -> HLG SCENE LINEAR
        pixel = hlg_dl_to_sl(pixel);

        // SCENE LINEAR -> HLG SIGNAL
        RgbPixel {
            red: hlg_sl_to_e(pixel.red).clamp(0.0, 1.0),
            green: hlg_sl_to_e(pixel.green).clamp(0.0, 1.0),
            blue: hlg_sl_to_e(pixel.blue).clamp(0.0, 1.0),
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
    bt1886: Bt1886,
}

impl PqSdrMapper {

    pub fn new(max_cll: f64, factor: f64) -> Self {
        Self {
            prepper: PqPrepper::new(max_cll, factor),
            bt1886: Bt1886::new(120.0, 0.0),
        }
    }

    pub fn map(&self, input: RgbPixel) -> RgbPixel {

        let mut pixel = self.prepper.prep(input);

        // TONE MAPPING TO SDR
        pixel = bt2446_c_tone_map(pixel);

        // MONOCHROME
        let y = pixel.to_yxy().y.clamp(0.0, 1.0);
        pixel = RgbPixel {
            red: y,
            green: y,
            blue: y,
        };

        // SDR LINEAR -> SDR GAMMA
        RgbPixel {
            red: self.bt1886.ieotf(pixel.red * 120.0).clamp(0.0, 1.0),
            green: self.bt1886.ieotf(pixel.green * 120.0).clamp(0.0, 1.0),
            blue: self.bt1886.ieotf(pixel.blue * 120.0).clamp(0.0, 1.0),
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
        let peak = max_cll / 10_000.0 * factor;
        let tone_mapper = Bt2408ToneMapper::new(peak);

        Self { factor, gamma, peak, tone_mapper }
    }

    fn prep(&self, input: RgbPixel) -> RgbPixel {

        let mut rgb_pixel = input;

        // PQ SIGNAL -> PQ DISPLAY LINEAR
        rgb_pixel = RgbPixel {
            red: pq_e_to_dl(rgb_pixel.red),
            green: pq_e_to_dl(rgb_pixel.green),
            blue: pq_e_to_dl(rgb_pixel.blue),
        };

        // SCALING
        rgb_pixel = (rgb_pixel.to_yxy() * self.factor).to_rgb();

        // TONE MAPPING
        if self.peak > 0.1 {
            rgb_pixel.red = self.tone_mapper.map(rgb_pixel.red);
            rgb_pixel.green = self.tone_mapper.map(rgb_pixel.green);
            rgb_pixel.blue = self.tone_mapper.map(rgb_pixel.blue);
        }

        // PQ DISPLAY LINEAR -> HLG DISPLAY LINEAR
        rgb_pixel *= 10.0;

        // GAMMA CORRECTION
        rgb_pixel = rgb_pixel.to_yxy().powf(1.0 / self.gamma).to_rgb();

        rgb_pixel
    }
}
