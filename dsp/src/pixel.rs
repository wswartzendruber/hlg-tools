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

use std::ops::{Add, Div, Mul, MulAssign};
use super::{RED_FACTOR, GREEN_FACTOR, BLUE_FACTOR};

const ONE_THIRD: f64 = 1.0 / 3.0;
const TWO_THIRDS: f64 = 2.0 / 3.0;

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

    pub fn y(&self) -> f64 {
        RED_FACTOR * self.red + GREEN_FACTOR * self.green + BLUE_FACTOR * self.blue
    }

    pub fn to_hsl(&self) -> HslPixel {

        let min_rgb = self.red.min(self.green.min(self.blue));
        let max_rgb = self.red.max(self.green.max(self.blue));
        let l = (min_rgb + max_rgb) / 2.0;
        let (mut h, s) = if min_rgb != max_rgb {
            let s = if l <= 0.5 {
                (max_rgb - min_rgb) / (max_rgb + min_rgb)
            } else {
                (max_rgb - min_rgb) / (2.0 - max_rgb - min_rgb)
            };
            let h = if self.red >= self.green && self.red >= self.blue {
                (self.green - self.blue) / (max_rgb - min_rgb)
            } else if self.green >= self.red && self.green >= self.blue {
                2.0 + (self.blue - self.red) / (max_rgb - min_rgb)
            } else if self.blue >= self.red && self.blue >= self.green {
                4.0 + (self.red - self.green) / (max_rgb - min_rgb)
            } else {
                unreachable!("Maximum between R, G, and B could not be determined.")
            };
            (h * 60.0, s)
        } else {
            (0.0, 0.0)
        };

        if h < 0.0 {
            h += 360.0;
        }

        HslPixel {
            hue: h,
            saturation: s,
            luminance: l,
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
// HSL
//

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HslPixel {
    pub hue: f64,
    pub saturation: f64,
    pub luminance: f64,
}

impl HslPixel {

    pub fn new_l(l: f64) -> Self {
        Self {
            hue: 0.0,
            saturation: 0.0,
            luminance: l,
        }
    }

    pub fn new_hsl(h: f64, s: f64, l: f64) -> Self {
        Self {
            hue: h,
            saturation: s,
            luminance: l,
        }
    }

    pub fn to_rgb(&self) -> RgbPixel {

        if self.saturation == 0.0 {
            RgbPixel {
                red: self.luminance,
                green: self.luminance,
                blue: self.luminance,
            }
        } else {

            let one_third = 1.0 / 3.0;
            let t1 = if self.luminance < 0.5 {
                self.luminance * (1.0 + self.saturation)
            } else {
                self.luminance + self.saturation - (self.luminance * self.saturation)
            };
            let t2 = 2.0 * self.luminance - t1;
            let th = self.hue / 360.0;
            let mut tr = th + ONE_THIRD;
            let mut tg = th;
            let mut tb = th - ONE_THIRD;

            if tr < 0.0 {
                tr += 1.0;
            } else if tr > 1.0 {
                tr -= 1.0;
            }
            if tg < 0.0 {
                tg += 1.0;
            } else if tg > 1.0 {
                tg -= 1.0;
            }
            if tb < 0.0 {
                tb += 1.0;
            } else if tb > 1.0 {
                tb -= 1.0;
            }

            RgbPixel {
                red: self.channel_checks(tr, t1, t2),
                green: self.channel_checks(tg, t1, t2),
                blue: self.channel_checks(tb, t1, t2),
            }
        }
    }

    fn channel_checks(&self, t: f64, t1: f64, t2: f64) -> f64 {
        if 6.0 * t < 1.0 {
            t2 + (t1 - t2) * 6.0 * t
        } else if 2.0 * t < 1.0 {
            t1
        } else if 3.0 * t < 2.0 {
            t2 + (t1 - t2) * (TWO_THIRDS - t) * 6.0
        } else {
            t2
        }
    }
}
