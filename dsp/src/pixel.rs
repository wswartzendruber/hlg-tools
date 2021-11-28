/*
 * SPDX-FileCopyrightText: 2021 William Swartzendruber <wswartzendruber@gmail.com>
 *
 * SPDX-License-Identifier: OSL-3.0
 */

#[cfg(test)]
mod tests;

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

    pub fn to_yxy(&self) -> YxyPixel {

        let x = 0.6370 * self.red + 0.1446 * self.green + 0.1689 * self.blue;
        let y = 0.2627 * self.red + 0.6780 * self.green + 0.0593 * self.blue;
        let z = 0.0000 * self.red + 0.0281 * self.green + 1.0610 * self.blue;

        if self.red == self.green && self.green == self.blue {
            YxyPixel {
                y,
                xc: 0.31270561916041584,
                yc: 0.3289906566653507,
            }
        } else {
            YxyPixel {
                y,
                xc: x / (x + y + z),
                yc: y / (x + y + z),
            }
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
// YXY
//

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct YxyPixel {
    pub y: f64,
    pub xc: f64,
    pub yc: f64,
}

impl YxyPixel {

    pub fn to_rgb(&self) -> RgbPixel {

        let x = self.xc / self.yc * self.y;
        let z = (1.0 - self.xc - self.yc) / self.yc * self.y;

        RgbPixel {
            // BT.2446-0 has coefficients defined to five significant digits. I found them to
            // produce unsatisfactory results. I therefore calculated the inverse coefficients
            // using the forward ones.
            red: 1.716502508360627 * x - 0.355584689096763 * self.y - 0.253375213570850 * z,
            green: -0.666625609145029 * x + 1.616446566522206 * self.y + 0.015775479726511 * z,
            blue: 0.017655211703087 * x - 0.042810696059636 * self.y + 0.942089263920532 * z,
        }
    }

    pub fn powf(&self, n: f64) -> YxyPixel {
        YxyPixel {
            y: self.y.powf(n),
            xc: self.xc,
            yc: self.yc,
        }
    }
}

impl Mul<f64> for YxyPixel {

    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        YxyPixel {
            y: self.y * rhs,
            xc: self.xc,
            yc: self.yc,
        }
    }
}
