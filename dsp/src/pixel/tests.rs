/*
 * Copyright 2023 William Swartzendruber
 *
 * To the extent possible under law, the person who associated CC0 with this file has waived all
 * copyright and related or neighboring rights to this file.
 *
 * You should have received a copy of the CC0 legalcode along with this work. If not, see
 * <http://creativecommons.org/publicdomain/zero/1.0/>.
 *
 * SPDX-License-Identifier: CC0-1.0
 */

use super::*;
use assert_approx_eq::assert_approx_eq;

const DIFF: f64 = 0.0000000001;

#[test]
fn test_rgb_hsl_roundtrip() {

    for r in 0..=1_000 {
        for g in 0..=1_000 {
            for b in 0..=1_000 {

                let in_rgb_pixel = RgbPixel {
                    red: r as f64 / 1_000.0,
                    green: g as f64 / 1_000.0,
                    blue: b as f64 / 1_000.0,
                };
                let out_rgb_pixel = in_rgb_pixel.to_hsl().to_rgb();

                assert_approx_eq!(in_rgb_pixel.red, out_rgb_pixel.red, DIFF);
                assert_approx_eq!(in_rgb_pixel.green, out_rgb_pixel.green, DIFF);
                assert_approx_eq!(in_rgb_pixel.blue, out_rgb_pixel.blue, DIFF);
            }
        }
    }
}
