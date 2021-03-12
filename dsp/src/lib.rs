/*
 * Copyright © 2021 William Swartzendruber
 * Licensed under the Open Software License version 3.0
 */

#[cfg(test)]
mod tests;

pub mod tf;
pub mod tm;

use tf::{hlg_oetf, pq_eotf, pq_hlg_iootf};
use tm::ToneMapper;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixel {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

pub struct PqHlgMapper {
    factor: f64,
    peak: f64,
    tone_mapper: ToneMapper,
}

impl PqHlgMapper {

    pub fn new(ref_white: f64, max_channel: f64) -> Self {

        let factor = 203.0 / ref_white;
        let peak = max_channel * factor;
        let tone_mapper = ToneMapper::new(peak);

        Self { factor, peak, tone_mapper }
    }

    pub fn map(&self, input: Pixel) -> Pixel {

        let mut pixel = input;

        // GAMMA -> LINEAR
        pixel = Pixel {
            red: pq_eotf(pixel.red),
            green: pq_eotf(pixel.green),
            blue: pq_eotf(pixel.blue),
        };

        // REFERENCE WHITE ADJUSTMENT
        pixel = Pixel {
            red: pixel.red * self.factor,
            green: pixel.green * self.factor,
            blue: pixel.blue * self.factor,
        };

        // TONE MAPPING
        if self.peak > 0.1 {
            pixel = Pixel {
                red: self.tone_mapper.map(pixel.red),
                green: self.tone_mapper.map(pixel.green),
                blue: self.tone_mapper.map(pixel.blue),
            }
        }

        // PQ -> HLG CONVERSION
        pixel = pq_hlg_iootf(pixel);

        // LINEAR -> GAMMA
        let hlg_gamma_pixel = Pixel {
            red: hlg_oetf(pixel.red),
            green: hlg_oetf(pixel.green),
            blue: hlg_oetf(pixel.blue),
        };

        hlg_gamma_pixel
    }
}
