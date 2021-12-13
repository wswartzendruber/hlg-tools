/*
 * SPDX-FileCopyrightText: 2021 William Swartzendruber <wswartzendruber@gmail.com>
 *
 * SPDX-License-Identifier: OSL-3.0
 */

use std::ops::{Mul, MulAssign};

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

    pub fn clamp(&self) -> Self {
        Self {
            red: self.red.clamp(0.0, 1.0),
            green: self.green.clamp(0.0, 1.0),
            blue: self.blue.clamp(0.0, 1.0),
        }
    }

    pub fn y(&self) -> f64 {
        0.2627 * self.red + 0.6780 * self.green + 0.0593 * self.blue
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
