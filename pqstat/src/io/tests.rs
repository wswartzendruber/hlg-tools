/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
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
    let mut frame = vec![Pixel { red: 0.0, green: 0.0, blue: 0.0 }; 6];

    read_frame(&mut cursor, &mut frame).unwrap();

    assert_eq!(frame[0], Pixel {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    });
    assert_eq!(frame[1], Pixel {
        red: 1.0,
        green: 0.0,
        blue: 0.0,
    });
    assert_eq!(frame[2], Pixel {
        red: 0.0,
        green: 1.0,
        blue: 0.0,
    });
    assert_eq!(frame[3], Pixel {
        red: 0.0,
        green: 0.0,
        blue: 1.0,
    });
    assert_eq!(frame[4], Pixel {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    });
    assert_eq!(frame[5], Pixel {
        red: 0.49999237048905165,
        green: 0.49999237048905165,
        blue: 0.49999237048905165
    });
}
