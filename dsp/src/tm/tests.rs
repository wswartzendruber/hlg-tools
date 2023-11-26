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
use more_asserts::{assert_gt, assert_lt};

const DIFF: f64 = 0.0000000001;

#[test]
fn test_bt2408_overrun_peak_1_000() {

    let pq_ootf = Bt2408ToneMapper::new(0.1, ToneMapMethod::Rgb);

    for i in 0..1_000 {
        assert_lt!(pq_ootf.map(RgbPixel::new_y(i as f64 / 10_000.0)).y(), 0.1);
    }

    for i in 1_000..10_000 {
        assert_gt!(pq_ootf.map(RgbPixel::new_y(i as f64 / 10_000.0)).y(), 0.1);
    }
}

#[test]
fn test_bt2408_overrun_peak_4_000() {

    let pq_ootf = Bt2408ToneMapper::new(0.4, ToneMapMethod::Rgb);

    for i in 0..4_000 {
        assert_lt!(pq_ootf.map(RgbPixel::new_y(i as f64 / 10_000.0)).y(), 0.1);
    }

    for i in 4_000..10_000 {
        assert_gt!(pq_ootf.map(RgbPixel::new_y(i as f64 / 10_000.0)).y(), 0.1);
    }
}

#[test]
fn test_bt2408_overrun_peak_10_000() {

    let pq_ootf = Bt2408ToneMapper::new(1.0, ToneMapMethod::Rgb);

    for i in 0..9_999 {
        assert_lt!(pq_ootf.map(RgbPixel::new_y(i as f64 / 10_000.0)).y(), 0.1);
    }
}

#[test]
fn test_bt2408_ootf_peak_1000() {

    let pq_ootf = Bt2408ToneMapper::new(0.1, ToneMapMethod::Rgb);

    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.00)).y(), 0.0, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.01)).y(), 0.01, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.02)).y(), 0.02, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.03)).y(), 0.03, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.04)).y(), 0.04, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.05)).y(), 0.05, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.06)).y(), 0.06, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.07)).y(), 0.07, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.08)).y(), 0.08, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.09)).y(), 0.09, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.10)).y(), 0.1, DIFF);
}

#[test]
fn test_bt2408_ootf_peak_2000() {

    let pq_ootf = Bt2408ToneMapper::new(0.2, ToneMapMethod::Rgb);

    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.00)).y(), 0.0, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.02)).y(), 0.02, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.04)).y(), 0.04, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.06)).y(), 0.06, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.08)).y(), 0.0788733929849, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.10)).y(), 0.0902325902087, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.12)).y(), 0.0959704921902, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.14)).y(), 0.0986105869303, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.16)).y(), 0.0996582983608, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.18)).y(), 0.099964016835, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.20)).y(), 0.1, DIFF);
}

#[test]
fn test_bt2408_ootf_peak_4000() {

    let pq_ootf = Bt2408ToneMapper::new(0.4, ToneMapMethod::Rgb);

    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.0)).y(), 0.0, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.04)).y(), 0.04, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.08)).y(), 0.0725237577607, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.12)).y(), 0.087451059744, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.16)).y(), 0.0942822121197, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.20)).y(), 0.0974937173873, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.24)).y(), 0.0989933164328, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.28)).y(), 0.0996575369944, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.32)).y(), 0.0999163530966, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.36)).y(), 0.0999912239036, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.4)).y(), 0.1, DIFF);
}

#[test]
fn test_bt2408_ootf_peak_8000() {

    let pq_ootf = Bt2408ToneMapper::new(0.8, ToneMapMethod::Rgb);

    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.0)).y(), 0.0, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.08)).y(), 0.0659387923192, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.16)).y(), 0.0868922897571, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.24)).y(), 0.0943498933538, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.32)).y(), 0.0974933977663, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.40)).y(), 0.0989165570687, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.48)).y(), 0.0995683080591, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.56)).y(), 0.0998538779943, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.64)).y(), 0.0999644309626, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.72)).y(), 0.0999962776243, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.8)).y(), 0.1, DIFF);
}

#[test]
fn test_bt2408_ootf_peak_10000() {

    let pq_ootf = Bt2408ToneMapper::new(1.0, ToneMapMethod::Rgb);

    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.0)).y(), 0.0, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.1)).y(), 0.0713332140595, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.2)).y(), 0.0892739186715, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.3)).y(), 0.0954237522859, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.4)).y(), 0.0979798879174, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.5)).y(), 0.0991292928898, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.6)).y(), 0.0996536883985, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.7)).y(), 0.0998829221806, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.8)).y(), 0.0999715271039, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(0.9)).y(), 0.0999970224486, DIFF);
    assert_approx_eq!(pq_ootf.map(RgbPixel::new_y(1.0)).y(), 0.1, DIFF);
}
