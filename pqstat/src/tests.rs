/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/0xFFFF/
 */

use super::{Pixel, frame_stats};

#[test]
fn test_frame_stats_black() {

    let frame = vec![
        Pixel { red: 0x0000, green: 0x0000, blue: 0x0000},
        Pixel { red: 0x0000, green: 0x0000, blue: 0x0000},
        Pixel { red: 0x0000, green: 0x0000, blue: 0x0000},
        Pixel { red: 0x0000, green: 0x0000, blue: 0x0000},
        Pixel { red: 0x0000, green: 0x0000, blue: 0x0000},
        Pixel { red: 0x0000, green: 0x0000, blue: 0x0000},
    ];
    let frame_stats = frame_stats(&frame);

    assert_eq!(frame_stats.max_cll, 0);
}

#[test]
fn test_frame_stats_red_rw() {

    let frame = vec![
        Pixel { red: 0x0000, green: 0x0000, blue: 0x0000 },
        Pixel { red: 0x4000, green: 0x0000, blue: 0x0000 },
        Pixel { red: 0x8000, green: 0x0000, blue: 0x0000 },
        Pixel { red: 0x94A7, green: 0x0000, blue: 0x0000 },
    ];
    let frame_stats = frame_stats(&frame);

    assert_eq!(frame_stats.max_cll, 203);
}

#[test]
fn test_frame_stats_green_rw() {

    let frame = vec![
        Pixel { red: 0x0000, green: 0x0000, blue: 0x0000 },
        Pixel { red: 0x0000, green: 0x4000, blue: 0x0000 },
        Pixel { red: 0x0000, green: 0x8000, blue: 0x0000 },
        Pixel { red: 0x0000, green: 0x94A7, blue: 0x0000 },
    ];
    let frame_stats = frame_stats(&frame);

    assert_eq!(frame_stats.max_cll, 203);
}

#[test]
fn test_frame_stats_blue_rw() {

    let frame = vec![
        Pixel { red: 0x0000, green: 0x0000, blue: 0x0000 },
        Pixel { red: 0x0000, green: 0x0000, blue: 0x4000 },
        Pixel { red: 0x0000, green: 0x0000, blue: 0x8000 },
        Pixel { red: 0x0000, green: 0x0000, blue: 0x94A7 },
    ];
    let frame_stats = frame_stats(&frame);

    assert_eq!(frame_stats.max_cll, 203);
}

#[test]
fn test_frame_stats_white_rw() {

    let frame = vec![
        Pixel { red: 0x0000, green: 0x0000, blue: 0x0000 },
        Pixel { red: 0x4000, green: 0x4000, blue: 0x4000 },
        Pixel { red: 0x8000, green: 0x8000, blue: 0x8000 },
        Pixel { red: 0x94A7, green: 0x94A7, blue: 0x94A7 },
    ];
    let frame_stats = frame_stats(&frame);

    assert_eq!(frame_stats.max_cll, 203);
}

#[test]
fn test_frame_stats_red_mw() {

    let frame = vec![
        Pixel { red: 0x0000, green: 0x0000, blue: 0x0000 },
        Pixel { red: 0x4000, green: 0x0000, blue: 0x0000 },
        Pixel { red: 0x8000, green: 0x0000, blue: 0x0000 },
        Pixel { red: 0xA000, green: 0x0000, blue: 0x0000 },
        Pixel { red: 0xD000, green: 0x0000, blue: 0x0000 },
        Pixel { red: 0xFFFF, green: 0x0000, blue: 0x0000 },
    ];
    let frame_stats = frame_stats(&frame);

    assert_eq!(frame_stats.max_cll, 10_000);
}

#[test]
fn test_frame_stats_green_mw() {

    let frame = vec![
        Pixel { red: 0x0000, green: 0x0000, blue: 0x0000 },
        Pixel { red: 0x0000, green: 0x4000, blue: 0x0000 },
        Pixel { red: 0x0000, green: 0x8000, blue: 0x0000 },
        Pixel { red: 0x0000, green: 0xA000, blue: 0x0000 },
        Pixel { red: 0x0000, green: 0xD000, blue: 0x0000 },
        Pixel { red: 0x0000, green: 0xFFFF, blue: 0x0000 },
    ];
    let frame_stats = frame_stats(&frame);

    assert_eq!(frame_stats.max_cll, 10_000);
}

#[test]
fn test_frame_stats_blue_mw() {

    let frame = vec![
        Pixel { red: 0x0000, green: 0x0000, blue: 0x0000 },
        Pixel { red: 0x0000, green: 0x0000, blue: 0x4000 },
        Pixel { red: 0x0000, green: 0x0000, blue: 0x8000 },
        Pixel { red: 0x0000, green: 0x0000, blue: 0xA000 },
        Pixel { red: 0x0000, green: 0x0000, blue: 0xD000 },
        Pixel { red: 0x0000, green: 0x0000, blue: 0xFFFF },
    ];
    let frame_stats = frame_stats(&frame);

    assert_eq!(frame_stats.max_cll, 10_000);
}

#[test]
fn test_frame_stats_white_mw() {

    let frame = vec![
        Pixel { red: 0x0000, green: 0x0000, blue: 0x0000 },
        Pixel { red: 0x4000, green: 0x4000, blue: 0x4000 },
        Pixel { red: 0x8000, green: 0x8000, blue: 0x8000 },
        Pixel { red: 0xA000, green: 0xA000, blue: 0xA000 },
        Pixel { red: 0xD000, green: 0xD000, blue: 0xD000 },
        Pixel { red: 0xFFFF, green: 0xFFFF, blue: 0xFFFF },
    ];
    let frame_stats = frame_stats(&frame);

    assert_eq!(frame_stats.max_cll, 10_000);
}
