/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

use super::{Pixel, frame_stats};

#[test]
fn test_frame_stats_black() {

    let frame = vec![
        Pixel { red: 0.0, green: 0.0, blue: 0.0},
        Pixel { red: 0.0, green: 0.0, blue: 0.0},
        Pixel { red: 0.0, green: 0.0, blue: 0.0},
        Pixel { red: 0.0, green: 0.0, blue: 0.0},
        Pixel { red: 0.0, green: 0.0, blue: 0.0},
        Pixel { red: 0.0, green: 0.0, blue: 0.0},
    ];
    let frame_stats = frame_stats(&frame);

    assert_eq!(frame_stats.max_cll, 0.0);
    assert_eq!(frame_stats.max_channel, 0.0);
}

#[test]
fn test_frame_stats_red() {

    let frame = vec![
        Pixel { red: 0.0, green: 0.0, blue: 0.0 },
        Pixel { red: 0.2, green: 0.0, blue: 0.0 },
        Pixel { red: 0.4, green: 0.0, blue: 0.0 },
        Pixel { red: 0.6, green: 0.0, blue: 0.0 },
        Pixel { red: 0.8, green: 0.0, blue: 0.0 },
        Pixel { red: 1.0, green: 0.0, blue: 0.0 },
    ];
    let frame_stats = frame_stats(&frame);

    assert_eq!(frame_stats.max_cll, 0.2627);
    assert_eq!(frame_stats.max_channel, 1.0);
}

#[test]
fn test_frame_stats_green() {

    let frame = vec![
        Pixel { red: 0.0, green: 0.0, blue: 0.0 },
        Pixel { red: 0.0, green: 0.2, blue: 0.0 },
        Pixel { red: 0.0, green: 0.4, blue: 0.0 },
        Pixel { red: 0.0, green: 0.6, blue: 0.0 },
        Pixel { red: 0.0, green: 0.8, blue: 0.0 },
        Pixel { red: 0.0, green: 1.0, blue: 0.0 },
    ];
    let frame_stats = frame_stats(&frame);

    assert_eq!(frame_stats.max_cll, 0.6780);
    assert_eq!(frame_stats.max_channel, 1.0);
}

#[test]
fn test_frame_stats_blue() {

    let frame = vec![
        Pixel { red: 0.0, green: 0.0, blue: 0.0 },
        Pixel { red: 0.0, green: 0.0, blue: 0.2 },
        Pixel { red: 0.0, green: 0.0, blue: 0.4 },
        Pixel { red: 0.0, green: 0.0, blue: 0.6 },
        Pixel { red: 0.0, green: 0.0, blue: 0.8 },
        Pixel { red: 0.0, green: 0.0, blue: 1.0 },
    ];
    let frame_stats = frame_stats(&frame);

    assert_eq!(frame_stats.max_cll, 0.0593);
    assert_eq!(frame_stats.max_channel, 1.0);
}

#[test]
fn test_frame_stats_white() {

    let frame = vec![
        Pixel { red: 0.0, green: 0.0, blue: 0.0 },
        Pixel { red: 0.2, green: 0.2, blue: 0.2 },
        Pixel { red: 0.4, green: 0.4, blue: 0.4 },
        Pixel { red: 0.6, green: 0.6, blue: 0.6 },
        Pixel { red: 0.8, green: 0.8, blue: 0.8 },
        Pixel { red: 1.0, green: 1.0, blue: 1.0 },
    ];
    let frame_stats = frame_stats(&frame);

    assert_eq!(frame_stats.max_cll, 1.0);
    assert_eq!(frame_stats.max_channel, 1.0);
}
