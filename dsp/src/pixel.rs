/*
 * This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a
 * copy of the MPL was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 *
 * Copyright 2021 William Swartzendruber
 *
 * SPDX-License-Identifier: MPL-2.0
 */

use std::ops::{Mul, MulAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixel {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Pixel {

    pub fn clamp(&self) -> Self {
        self.with_each_channel(|x| x.clamp(0.0, 1.0))
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
        0.2627 * self.red + 0.6780 * self.green + 0.0593 * self.blue
    }
}

impl Mul<f64> for Pixel {

    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        self.with_each_channel(|x| x * rhs)
    }
}

impl MulAssign<f64> for Pixel {

    fn mul_assign(&mut self, rhs: f64) {
        *self = self.with_each_channel(|x| x * rhs);
    }
}
