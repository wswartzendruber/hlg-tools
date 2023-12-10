/*
 * Copyright 2023 William Swartzendruber
 *
 * This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a
 * copy of the MPL was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 *
 * SPDX-License-Identifier: MPL-2.0
 */

#[cfg(test)]
mod tests;

use std::ops::{Mul, MulAssign};

pub const RED_FACTOR_2020: f64 = 0.2627;
pub const GREEN_FACTOR_2020: f64 = 0.6780;
pub const BLUE_FACTOR_2020: f64 = 0.0593;
pub const RED_FACTOR_709: f64 = 0.2126;
pub const GREEN_FACTOR_709: f64 = 0.7152;
pub const BLUE_FACTOR_709: f64 = 0.0722;

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

    pub fn y_bt2020(&self) -> f64 {
        RED_FACTOR_2020 * self.red
            + GREEN_FACTOR_2020 * self.green
            + BLUE_FACTOR_2020 * self.blue
    }

    pub fn y_bt709(&self) -> f64 {
        RED_FACTOR_709 * self.red
            + GREEN_FACTOR_709 * self.green
            + BLUE_FACTOR_709 * self.blue
    }

    pub fn bt2020_to_xyz(&self) -> XyzPixel {
        XyzPixel {
            x: 0.6370101914111008 * self.red
                + 0.14461502739696927 * self.green
                + 0.16884478119192986 * self.blue,
            y: 0.26272171736164046 * self.red
                + 0.6779892755022618 * self.green
                + 0.0592890071360975 * self.blue,
            z: 0.028072328847646908 * self.green
                + 1.060757671152353 * self.blue,
        }
    }

    pub fn bt709_to_xyz(&self) -> XyzPixel {
        XyzPixel {
            x: 0.4124564 * self.red
                + 0.3575761 * self.green
                + 0.1804375 * self.blue,
            y: 0.2126729 * self.red
                + 0.7151522 * self.green
                + 0.0721750 * self.blue,
            z: 0.0193339 * self.red
                + 0.1191920 * self.green
                + 0.9503041 * self.blue,
        }
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

    pub fn to_rgb_bt2020(&self) -> RgbPixel {
        RgbPixel {
            red: 1.7165106697619736 * self.x
                - 0.3556416699867159 * self.y
                - 0.25334554182190727 * self.z,
            green: -0.6666930011826243 * self.x
                + 1.6165022083469107 * self.y
                + 0.015768750389995017 * self.z,
            blue: 0.01764363876745901 * self.x
                - 0.04277978166904462 * self.y
                + 0.9423050727200186 * self.z,
        }
    }

    pub fn to_rgb_bt709(&self) -> RgbPixel {
        RgbPixel {
            red: 3.2404542 * self.x
                - 1.5371385 * self.y
                - 0.4985314 * self.z,
            green: -0.9692660 * self.x
                + 1.8760108 * self.y
                + 0.0415560 * self.z,
            blue: 0.0556434 * self.x
                - 0.2040259 * self.y
                + 1.0572252 * self.z,
        }
    }

    pub fn to_oklab(&self) -> OklabPixel {

        let l = 0.8189330101 * self.x + 0.3618667424 * self.y - 0.1288597137 * self.z;
        let m = 0.0329845436 * self.x + 0.9293118715 * self.y + 0.0361456387 * self.z;
        let s = 0.0482003018 * self.x + 0.2643662691 * self.y + 0.6338517070 * self.z;

        let l_ = l.powf(0.3333333333333333);
        let m_ = m.powf(0.3333333333333333);
        let s_ = s.powf(0.3333333333333333);

        OklabPixel {
            l: 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_,
            a: 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_,
            b: 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_,
        }
    }
}

//
// Oklab
//

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OklabPixel {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

impl OklabPixel {

    pub fn monochrome(&self) -> Self {
        OklabPixel {
            l: self.l,
            a: 0.0,
            b: 0.0,
        }
    }

    pub fn to_xyz(&self) -> XyzPixel {

        let l_ =
            0.99999999845051981432 * self.l
            + 0.39633779217376785678 * self.a
            + 0.21580375806075880339 * self.b;
        let m_ =
            1.0000000088817607767 * self.l
            - 0.1055613423236563494 * self.a
            - 0.063854174771705903402 * self.b;
        let s_ =
            1.0000000546724109177 * self.l
            - 0.089484182094965759684 * self.a
            - 1.2914855378640917399 * self.b;
        let l = l_.powf(3.0);
        let m = m_.powf(3.0);
        let s = s_.powf(3.0);

        XyzPixel {
            x:
                1.227013851103521026 * l
                - 0.5577999806518222383 * m
                + 0.28125614896646780758 * s,
            y:
                -0.040580178423280593977 * l
                + 1.1122568696168301049 * m
                - 0.071676678665601200577 * s,
            z:
                -0.076381284505706892869 * l
                - 0.42148197841801273055 * m
                + 1.5861632204407947575 * s,
        }
    }
}

impl Mul<f64> for OklabPixel {

    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            l: self.l * rhs,
            a: self.a * rhs,
            b: self.b * rhs,
        }
    }
}
