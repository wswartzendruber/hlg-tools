/*
 * SPDX-FileCopyrightText: 2021 William Swartzendruber <wswartzendruber@gmail.com>
 *
 * SPDX-License-Identifier: OSL-3.0
 */

#[cfg(test)]
mod tests;

pub mod tf;
pub mod tm;

use std::ops::{Mul, MulAssign};

use tf::{hlg_sl_to_e, pq_e_to_dl, hlg_dl_to_sl, sdr_o_to_e};
use tm::{bt2446_c_tone_map, Bt2408ToneMapper};

pub fn if_nan(value: f64, fallback: f64) -> f64 {
    if !value.is_nan() {
        value
    } else {
        fallback
    }
}

//
// PIXEL
//

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

    pub fn powf(&self, n: f64) -> Pixel {
        Pixel {
            red: self.red.powf(n),
            green: self.green.powf(n),
            blue: self.blue.powf(n),
        }
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
    prepper: PqPrepper,
}

impl PqHlgMapper {

    pub fn new(max_cll: f64, factor: f64) -> Self {
        Self { prepper: PqPrepper::new(max_cll, factor) }
    }

    pub fn map(&self, input: Pixel) -> Pixel {

        let mut pixel = self.prepper.prep(input);

        // PQ DISPLAY LINEAR -> HLG SCENE LINEAR
        pixel = hlg_dl_to_sl(pixel);

        // SCENE LINEAR -> HLG SIGNAL
        Pixel {
            red: hlg_sl_to_e(pixel.red).min(1.0),
            green: hlg_sl_to_e(pixel.green).min(1.0),
            blue: hlg_sl_to_e(pixel.blue).min(1.0),
        }
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
    prepper: PqPrepper,
}

impl PqSdrMapper {

    pub fn new(max_cll: f64, factor: f64) -> Self {
        Self { prepper: PqPrepper::new(max_cll, factor) }
    }

    pub fn map(&self, input: Pixel) -> Pixel {

        let mut pixel = self.prepper.prep(input);

        // MONOCHROME
        let y = pixel.y();
        pixel = Pixel {
            red: y,
            green: y,
            blue: y,
        };

        // TONE MAPPING TO SDR
        pixel = bt2446_c_tone_map(pixel * 10.0);

        // SDR LINEAR -> SDR GAMMA
        Pixel {
            red: sdr_o_to_e(pixel.red).min(1.0),
            green: sdr_o_to_e(pixel.green).min(1.0),
            blue: sdr_o_to_e(pixel.blue).min(1.0),
        }
    }
}

impl Mapper for PqSdrMapper {

    fn map(&self, input: Pixel) -> Pixel {
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

    fn prep(&self, input: Pixel) -> Pixel {

        let mut pixel = input;

        // PQ SIGNAL -> DISPLAY LINEAR
        pixel = Pixel {
            red: pq_e_to_dl(pixel.red),
            green: pq_e_to_dl(pixel.green),
            blue: pq_e_to_dl(pixel.blue),
        };

        // REFERENCE WHITE ADJUSTMENT
        pixel *= self.factor;
        pixel = pixel.powf(1.0 / self.gamma);

        // TONE MAPPING
        if self.peak > 0.1 {

            let y1 = pixel.y();
            let y2 = self.tone_mapper.map(y1);
            let r = if y1 == 0.0 { 0.0 } else { y2 / y1 };

            pixel *= r;
        }

        pixel
    }
}
