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

use pixel::Pixel;
use tf::{hlg_sl_to_e, pq_e_to_dl, hlg_dl_to_sl, sdr_o_to_e};
use tm::{sdn_tone_map, Bt2390ToneMapper};

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
    tone_mapper: Bt2390ToneMapper,
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
        let tone_mapper = Bt2390ToneMapper::new(peak);

        Self { factor, peak, tone_mapper }
    }

    pub fn map(&self, input: Pixel) -> Pixel {

        let mut pixel = input;

        // PQ SIGNAL -> DISPLAY LINEAR
        pixel = Pixel {
            red: pq_e_to_dl(pixel.red),
            green: pq_e_to_dl(pixel.green),
            blue: pq_e_to_dl(pixel.blue),
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

        // PQ DISPLAY LINEAR -> HLG SCENE LINEAR
        pixel = hlg_dl_to_sl(pixel);

        // SCENE LINEAR -> HLG SIGNAL
        let hlg_gamma_pixel = Pixel {
            red: hlg_sl_to_e(pixel.red).min(1.0),
            green: hlg_sl_to_e(pixel.green).min(1.0),
            blue: hlg_sl_to_e(pixel.blue).min(1.0),
        };

        hlg_gamma_pixel
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
    tone_mapper: Bt2390ToneMapper,
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
        let tone_mapper = Bt2390ToneMapper::new(peak);

        Self { factor, peak, tone_mapper }
    }

    pub fn map(&self, input: Pixel) -> Pixel {

        let mut pixel = input;

        // PQ SIGNAL -> DISPLAY LINEAR
        pixel = Pixel {
            red: pq_e_to_dl(pixel.red),
            green: pq_e_to_dl(pixel.green),
            blue: pq_e_to_dl(pixel.blue),
        };

        // REFERENCE WHITE ADJUSTMENT
        pixel *= self.factor;

        // TONE MAPPING (TO 1,000 NITS)
        if self.peak > 0.1 {

            let y1 = pixel.y();
            let y2 = self.tone_mapper.map(y1);
            let r = if y1 == 0.0 { 0.0 } else { y2 / y1 };

            pixel *= r;
        }

        // MONOCHROME
        let mut y = pixel.y();

        // TONE MAPPING (FROM 1,000 NITS TO 100 NITS)
        y = sdn_tone_map(y * 10.0);

        // SDR LINEAR -> SDR GAMMA
        let sdr_gamma_pixel = Pixel {
            red: sdr_o_to_e(y).min(1.0),
            green: sdr_o_to_e(y).min(1.0),
            blue: sdr_o_to_e(y).min(1.0),
        };

        sdr_gamma_pixel
    }
}

impl Mapper for PqSdrMapper {

    fn map(&self, input: Pixel) -> Pixel {
        self.map(input)
    }
}
