/*
 * SPDX-FileCopyrightText: 2021 William Swartzendruber <wswartzendruber@gmail.com>
 *
 * SPDX-License-Identifier: CC0-1.0
 */

use super::*;

use assert_approx_eq::assert_approx_eq;

const DIFF: f64 = 0.0001;

#[test]
fn test_rgb_xyz_round_trip() {

    let in_rgb_pixels = vec![
        RgbPixel {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        },
        RgbPixel {
            red: 0.5,
            green: 0.5,
            blue: 0.5,
        },
        RgbPixel {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        },
        RgbPixel {
            red: 0.0,
            green: 0.5,
            blue: 1.0,
        },
        RgbPixel {
            red: 1.0,
            green: 0.5,
            blue: 0.0,
        },
    ];

    for in_rgb_pixel in in_rgb_pixels.iter() {

        let out_rgb_pixel = in_rgb_pixel.to_xyz().to_rgb();

        assert_approx_eq!(in_rgb_pixel.red, out_rgb_pixel.red, DIFF);
        assert_approx_eq!(in_rgb_pixel.green, out_rgb_pixel.green, DIFF);
        assert_approx_eq!(in_rgb_pixel.blue, out_rgb_pixel.blue, DIFF);
    }
}
