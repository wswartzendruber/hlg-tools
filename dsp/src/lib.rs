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
use tf::{hlg_sl_to_e, pq_e_to_dl, hlg_dl_to_sl};
use tm::Bt2408ToneMapper;

pub struct PqHlgMapper {
    factor: f64,
    peak: f64,
    tone_mapper: Bt2408ToneMapper,
}

impl PqHlgMapper {

    pub fn new(max_cll: f64, factor: f64) -> Self {

        let peak = max_cll / 10_000.0 * factor;
        let tone_mapper = Bt2408ToneMapper::new(peak);

        Self { factor, peak, tone_mapper }
    }

    pub fn map(&self, input: RgbPixel) -> RgbPixel {

        let mut pixel = input.clamp();

        // PQ SIGNAL -> PQ DISPLAY LINEAR
        pixel = RgbPixel {
            red: pq_e_to_dl(pixel.red),
            green: pq_e_to_dl(pixel.green),
            blue: pq_e_to_dl(pixel.blue),
        }.clamp();

        // SCALING
        pixel *= self.factor;

        // TONE MAPPING
        if self.peak > 0.1 {
            pixel.red = self.tone_mapper.map(pixel.red);
            pixel.green = self.tone_mapper.map(pixel.green);
            pixel.blue = self.tone_mapper.map(pixel.blue);
        }

        // PQ DISPLAY LINEAR -> HLG DISPLAY LINEAR
        pixel = (pixel * 10.0).clamp();

        // HLG DISPLAY LINEAR -> HLG SCENE LINEAR
        pixel = hlg_dl_to_sl(pixel).clamp();

        // HLG SCENE LINEAR -> HLG SIGNAL
        RgbPixel {
            red: hlg_sl_to_e(pixel.red),
            green: hlg_sl_to_e(pixel.green),
            blue: hlg_sl_to_e(pixel.blue),
        }.clamp()
    }
}
