/*
 * SPDX-FileCopyrightText: 2021 William Swartzendruber <wswartzendruber@gmail.com>
 *
 * SPDX-License-Identifier: CC0-1.0
 */

use super::*;

use assert_approx_eq::assert_approx_eq;

const DIFF: f64 = 0.0000000001;

#[test]
fn test_rgb_yxy_round_trip() {

    for r in 0..1_000 {
        for g in 0..1_000 {
            for b in 0..1_000 {

                let in_rgb_pixel = RgbPixel {
                    red: r as f64 / 1_000.0,
                    green: g as f64 / 1_000.0,
                    blue: b as f64 / 1_000.0,
                };
                let out_rgb_pixel = in_rgb_pixel.to_yxy().to_rgb();

                assert_approx_eq!(in_rgb_pixel.red, out_rgb_pixel.red, DIFF);
                assert_approx_eq!(in_rgb_pixel.green, out_rgb_pixel.green, DIFF);
                assert_approx_eq!(in_rgb_pixel.blue, out_rgb_pixel.blue, DIFF);
            }
        }
    }
}
