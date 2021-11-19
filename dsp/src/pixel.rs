/*
 * SPDX-FileCopyrightText: 2021 William Swartzendruber <wswartzendruber@gmail.com>
 *
 * SPDX-License-Identifier: OSL-3.0
 */

#[cfg(test)]
mod tests;

use std::ops::{Mul, MulAssign};

const XN: f64 = 0.312713;
const YN: f64 = 0.329016;

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

    pub fn to_xyz(&self) -> XyzPixel {
        XyzPixel {
            x: 0.6370 * self.red + 0.1446 * self.green + 0.1689 * self.blue,
            y: 0.2627 * self.red + 0.6780 * self.green + 0.0593 * self.blue,
            z: 0.0000 * self.red + 0.0281 * self.green + 1.0610 * self.blue,
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
// XYZ
//

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XyzPixel {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl XyzPixel {

    pub fn to_rgb(&self) -> RgbPixel {
        RgbPixel {
            red: 1.7167 * self.x - 0.3557 * self.y - 0.2534 * self.z,
            green: -0.6667 * self.x + 1.6165 * self.y + 0.0158 * self.z,
            blue: 0.0176 * self.x - 0.0428 * self.y + 0.9421 * self.z,
        }
    }

    pub fn to_luv(&self) -> LuvPixel {

        let un = 4.0 * XN / (-2.0 * XN + 12.0 * YN + 3.0);
        let vn = 9.0 * YN / (-2.0 * XN + 12.0 * YN + 3.0);
        let u = 4.0 * self.x / (self.x + 15.0 * self.y + 3.0 * self.z);
        let v = 9.0 * self.y / (self.x + 15.0 * self.y + 3.0 * self.z);
        let l = 116.0 * (self.y / YN).powf(1.0 / 3.0) - 16.0;

        LuvPixel {
            l,
            u: 13.0 * l * (u - un),
            v: 13.0 * l * (v - vn),
        }
    }
}

//
// LUV
//

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LuvPixel {
    pub l: f64,
    pub u: f64,
    pub v: f64,
}

impl LuvPixel {

    pub fn to_xyz(&self) -> XyzPixel {
        XyzPixel {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
