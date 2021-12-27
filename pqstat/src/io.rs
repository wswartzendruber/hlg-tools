/*
 * This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a
 * copy of the MPL was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 *
 * Copyright 2021 William Swartzendruber
 *
 * SPDX-License-Identifier: MPL-2.0
 */

#[cfg(test)]
mod tests;

use std::io::{Read, Result};
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixel {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

pub fn read_frame(input: &mut dyn Read, frame: &mut [Pixel]) -> Result<()> {

    for pixel in frame.iter_mut() {
        *pixel = Pixel {
            red: input.read_u16::<LittleEndian>()?,
            green: input.read_u16::<LittleEndian>()?,
            blue: input.read_u16::<LittleEndian>()?,
        };
    }

    Ok(())
}
