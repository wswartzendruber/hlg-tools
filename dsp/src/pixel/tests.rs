/*
 * Copyright 2024 William Swartzendruber
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

const DIFF_2020: f64 = 0.00000000000001;
const DIFF_709: f64 = 0.000001;

#[test]
fn test_rgb_xyz_round_trip_bt2020() {

    const SIZE: usize = 128;

    for b in 0..=SIZE {
        for g in 0..=SIZE {
            for r in 0..=SIZE {

                let in_pixel = RgbPixel {
                    red: (r as f64) / (SIZE as f64),
                    green: (g as f64) / (SIZE as f64),
                    blue: (b as f64) / (SIZE as f64),
                };
                let out_pixel = in_pixel.bt2020_to_xyz().to_oklab().to_xyz().to_rgb_bt2020();

                assert_approx_eq!(out_pixel.red, in_pixel.red, DIFF_2020);
                assert_approx_eq!(out_pixel.green, in_pixel.green, DIFF_2020);
                assert_approx_eq!(out_pixel.blue, in_pixel.blue, DIFF_2020);
            }
        }
    }
}

#[test]
fn test_rgb_xyz_round_trip_bt709() {

    const SIZE: usize = 128;

    for b in 0..=SIZE {
        for g in 0..=SIZE {
            for r in 0..=SIZE {

                let in_pixel = RgbPixel {
                    red: (r as f64) / (SIZE as f64),
                    green: (g as f64) / (SIZE as f64),
                    blue: (b as f64) / (SIZE as f64),
                };
                let out_pixel = in_pixel.bt709_to_xyz().to_oklab().to_xyz().to_rgb_bt709();

                assert_approx_eq!(out_pixel.red, in_pixel.red, DIFF_709);
                assert_approx_eq!(out_pixel.green, in_pixel.green, DIFF_709);
                assert_approx_eq!(out_pixel.blue, in_pixel.blue, DIFF_709);
            }
        }
    }
}
