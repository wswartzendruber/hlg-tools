/*
 * SPDX-FileCopyrightText: 2021 William Swartzendruber <wswartzendruber@gmail.com>
 *
 * SPDX-License-Identifier: OSL-3.0
 */

// #[cfg(test)]
// mod tests;

use std::ops::{Mul, MulAssign};

const A: f64 = 0.2627;
const B: f64 = 0.6780;
const C: f64 = 0.0593;
const D: f64 = 1.8814;
const E: f64 = 1.4747;

//
// RGB
//

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RgbPixel {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl RgbPixel {

    pub fn to_ycbcr(&self) -> YcbcrPixel {

        let y = A * self.red + B * self.green + C * self.blue;

        YcbcrPixel {
            y,
            cb: (self.blue - y) / D,
            cr: (self.red - y) / E,
        }
    }
}

impl Mul<f64> for RgbPixel {

    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        RgbPixel {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl MulAssign<f64> for RgbPixel {

    fn mul_assign(&mut self, rhs: f64) {
        self.red *= rhs;
        self.green *= rhs;
        self.blue *= rhs;
    }
}

//
// YCBCR
//

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct YcbcrPixel {
    pub y: f64,
    pub cb: f64,
    pub cr: f64,
}

impl YcbcrPixel {

    pub fn to_rgb(&self) -> RgbPixel {
        RgbPixel {
            red: self.y + E * self.cr,
            green: self.y - (A * E / B) * self.cr - (C * D / B) * self.cb,
            blue: self.y + D * self.cb,
        }
    }
}
