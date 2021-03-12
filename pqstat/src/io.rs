/*
 * Copyright © 2021 William Swartzendruber
 * Licensed under the Open Software License version 3.0
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
