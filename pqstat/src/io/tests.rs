/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/0xFFFF/
 */

use std::io::Cursor;
use super::*;

#[test]
fn test_read_frame() {

    let mut cursor = Cursor::new(vec![
        0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8,
        0xFF_u8, 0xFF_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8,
        0x00_u8, 0x00_u8, 0xFF_u8, 0xFF_u8, 0x00_u8, 0x00_u8,
        0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0xFF_u8, 0xFF_u8,
        0xFF_u8, 0xFF_u8, 0xFF_u8, 0xFF_u8, 0xFF_u8, 0xFF_u8,
        0xFF_u8, 0x7F_u8, 0xFF_u8, 0x7F_u8, 0xFF_u8, 0x7F_u8,
    ]);
    let mut frame = vec![Pixel { red: 0, green: 0, blue: 0 }; 6];

    read_frame(&mut cursor, &mut frame).unwrap();

    assert_eq!(frame[0], Pixel {
        red: 0x0000,
        green: 0x0000,
        blue: 0x0000,
    });
    assert_eq!(frame[1], Pixel {
        red: 0xFFFF,
        green: 0x0000,
        blue: 0x0000,
    });
    assert_eq!(frame[2], Pixel {
        red: 0x0000,
        green: 0xFFFF,
        blue: 0x0000,
    });
    assert_eq!(frame[3], Pixel {
        red: 0x0000,
        green: 0x0000,
        blue: 0xFFFF,
    });
    assert_eq!(frame[4], Pixel {
        red: 0xFFFF,
        green: 0xFFFF,
        blue: 0xFFFF,
    });
    assert_eq!(frame[5], Pixel {
        red: 0x7FFF,
        green: 0x7FFF,
        blue: 0x7FFF
    });
}
