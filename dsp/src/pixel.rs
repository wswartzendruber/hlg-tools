/*
 * Copyright 2022 William Swartzendruber
 *
 * This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a
 * copy of the MPL was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 *
 * SPDX-License-Identifier: MPL-2.0
 */

#[cfg(test)]
mod tests;

use std::ops::{Add, Div, Mul, MulAssign};
use super::{RED_FACTOR, GREEN_FACTOR, BLUE_FACTOR};

//
// RGB (BT.2020)
//

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RgbPixel {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl RgbPixel {

    pub fn new_y(y: f64) -> Self {
        Self {
            red: y,
            green: y,
            blue: y,
        }
    }

    pub fn new_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
        }
    }

    pub fn clamp(&self, min: f64, max: f64) -> Self {
        self.with_each_channel(|x| x.clamp(min, max))
    }

    pub fn with_each_channel<F>(&self, f: F) -> Self
        where F: Fn(f64) -> f64 {
        Self {
            red: f(self.red),
            green: f(self.green),
            blue: f(self.blue),
        }
    }

    pub fn y(&self) -> f64 {
        RED_FACTOR * self.red + GREEN_FACTOR * self.green + BLUE_FACTOR * self.blue
    }

    pub fn to_xyz(&self) -> XyzPixel {
        XyzPixel {
            x: 0.6369580 * self.red + 0.1446169 * self.green + 0.1688810 * self.blue,
            y: 0.2627002 * self.red + 0.6779981 * self.green + 0.0593017 * self.blue,
            z: 0.0000000 * self.red + 0.0280727 * self.green + 1.0609851 * self.blue,
        }
    }
}

impl Add<RgbPixel> for RgbPixel {

    type Output = Self;

    fn add(self, rhs: RgbPixel) -> Self {
        Self {
            red: self.red + rhs.red,
            green: self.green + self.green,
            blue: self.blue + self.blue,
        }
    }
}

impl Div<f64> for RgbPixel {

    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        self.with_each_channel(|x| x / rhs)
    }
}

impl Mul<f64> for RgbPixel {

    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        self.with_each_channel(|x| x * rhs)
    }
}

impl MulAssign<f64> for RgbPixel {

    fn mul_assign(&mut self, rhs: f64) {
        *self = self.with_each_channel(|x| x * rhs);
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
            red: 1.7166512 * self.x - 0.3556708 * self.y - 0.2533663 * self.z,
            green: -0.6666844 * self.x + 1.6164812 * self.y + 0.0157685 * self.z,
            blue: 0.0176399 * self.x - 0.0427706 * self.y + 0.9421031 * self.z,
        }
    }
}
