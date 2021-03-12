/*
 * Copyright © 2021 William Swartzendruber
 * Licensed under the Open Software License version 3.0
 */

#[cfg(test)]
mod tests;

pub mod tf;
pub mod tm;

use std::ops::{Mul, MulAssign};

use tf::{hlg_oetf, pq_eotf, pq_hlg_iootf};
use tm::ToneMapper;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixel {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Pixel {

    pub fn y(&self) -> f64 {
        0.2627 * self.red + 0.6780 * self.green + 0.0593 * self.blue
    }
}

impl Mul<f64> for Pixel {

    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Pixel {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl MulAssign<f64> for Pixel {

    fn mul_assign(&mut self, rhs: f64) {
        self.red *= rhs;
        self.green *= rhs;
        self.blue *= rhs;
    }
}

pub struct PqHlgMapper {
    factor: f64,
    peak: f64,
    tone_mapper: ToneMapper,
}

impl PqHlgMapper {

    pub fn new(ref_white: f64, max_cll: f64) -> Self {

        let factor = 203.0 / ref_white;
        let peak = max_cll / 10_000.0 * factor;
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
        pixel *= self.factor;

        // TONE MAPPING
        if self.peak > 0.1 {

            let y1 = pixel.y();
            let y2 = self.tone_mapper.map(y1);
            let r = if y1 == 0.0 { 0.0 } else { y2 / y1 };

            pixel *= r;
        }

        // PQ -> HLG CONVERSION
        pixel = pq_hlg_iootf(pixel);

        // LINEAR -> GAMMA
        let hlg_gamma_pixel = Pixel {
            red: hlg_oetf(pixel.red).min(1.0),
            green: hlg_oetf(pixel.green).min(1.0),
            blue: hlg_oetf(pixel.blue).min(1.0),
        };

        hlg_gamma_pixel
    }
}
