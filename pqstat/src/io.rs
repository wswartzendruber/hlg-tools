/*
 * Copyright © 2021 William Swartzendruber
 * Licensed under the Open Software License version 3.0
 */

#[cfg(test)]
mod tests;

use std::io::{Read, Result};
use super::Pixel;
use byteorder::{LittleEndian, ReadBytesExt};

pub fn read_frame(input: &mut dyn Read, frame: &mut [Pixel]) -> Result<()> {

    for pixel in frame.iter_mut() {
        *pixel = Pixel {
            red: input.read_u16::<LittleEndian>()? as f64 / 65535.0,
            green: input.read_u16::<LittleEndian>()? as f64 / 65535.0,
            blue: input.read_u16::<LittleEndian>()? as f64 / 65535.0,
        };
    }

    Ok(())
}
