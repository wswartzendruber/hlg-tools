/*
 * Copyright © 2021 William Swartzendruber
 * Licensed under the Open Software License version 3.0
 */

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixel {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

pub struct PqHlgMapper {
    factor: f64,
    peak: f64,
    pq_ootf: PqOotf,
}

impl PqHlgMapper {

    pub fn new(ref_white: f64, max_channel: f64) -> Self {

        let factor = 203.0 / ref_white;
        let peak = max_channel * factor;
        let pq_ootf = PqOotf::new(peak);

        Self { factor, peak, pq_ootf }
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
                red: self.pq_ootf.map(pixel.red),
                green: self.pq_ootf.map(pixel.green),
                blue: self.pq_ootf.map(pixel.blue),
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

struct PqOotf {
    lwp: f64,
    ml: f64,
    ks: f64,
}

impl PqOotf {

    fn new(peak: f64) -> Self {

        let lwp = pq_oetf(peak);
        let ml = pq_oetf(0.10) / lwp;
        let ks = 1.5 * ml - 0.5;

        Self { lwp, ml, ks }
    }

    fn map(&self, e: f64) -> f64 {

        let e1 = pq_oetf(e) / self.lwp;
        let e2 =
            if e1 < self.ks {
                e1
            } else {
                p(self.ks, self.ml, e1)
            };

        pq_eotf(e2 * self.lwp).min(0.1)
    }
}

fn p(ks: f64, ml: f64, b: f64) -> f64 {

    let t = (b - ks) / (1.0 - ks);
    let t2 = t.powf(2.0);
    let t3 = t.powf(3.0);

    (2.0 * t3 - 3.0 * t2 + 1.0) * ks
    +
    (t3 - 2.0 * t2 + t) * (1.0 - ks)
    +
    (-2.0 * t3 + 3.0 * t2) * ml
}

fn pq_oetf(e: f64) -> f64 {

    //
    // ITU-R BT.2100-2
    // Table 4
    //

    (
        (0.8359375 + 18.8515625 * e.powf(0.1593017578125))
        /
        (1.0 + 18.6875 * e.powf(0.1593017578125))
    )
    .powf(78.84375)
}

pub fn pq_eotf(e: f64) -> f64 {

    //
    // ITU-R BT.2100-2
    // Table 4
    //

    (
        (e.powf(0.012683313515655966) - 0.8359375).max(0.0)
        /
        (18.8515625 - 18.6875 * e.powf(0.012683313515655966))
    )
    .powf(6.277394636015326)
}

fn pq_hlg_iootf(pixel: Pixel) -> Pixel {

    //
    // The BBC R&D Method of PQ to HLG Transcoding
    //

    let rd = pixel.red * 10.0;
    let gd = pixel.green * 10.0;
    let bd = pixel.blue * 10.0;
    let yg = (0.2627 * rd + 0.6780 * gd + 0.0593 * bd).powf(-0.166666667).min(f64::MAX);
    let red = rd * yg;
    let green = gd * yg;
    let blue = bd * yg;

    Pixel { red, green, blue }
}

fn hlg_oetf(o: f64) -> f64 {

    //
    // ITU-R BT.2100-2
    // Table 5
    //

    if o < 0.08333333333333333 {
        (3.0 * o).sqrt()
    } else {
        0.17883277 * (12.0 * o - 0.28466892).ln() + 0.559910729529562
    }
}
