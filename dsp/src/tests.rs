/*
 * Any copyright is dedicated to the Public Domain.
 *
 * Copyright 2022 William Swartzendruber
 *
 * SPDX-License-Identifier: CC0-1.0
 */

use super::{
    *,
    tm::ToneMapMethod,
};
use assert_approx_eq::assert_approx_eq;
use more_asserts::assert_gt;

const HDR_DIFF: f64 = 0.0000001;
const PQ_100_NITS: f64 = 0.508078421517399;
const PQ_492_NITS: f64 = 0.6749788198754852;
const PQ_1000_NITS: f64 = 0.751827096247041;
const PQ_1970_NITS: f64 = 0.8258028022027166;
const PQ_4000_NITS: f64 = 0.9025723933109373;
const PQ_4926_NITS: f64 = 0.9249489996796816;
const PQ_10000_NITS: f64 = 1.0;
const PQ_BLACK: f64 = 0.0;
const PQ_GREY_18: f64 = 0.3800322743334056;
const PQ_GREY_83: f64 = 0.557238560697735;
const PQ_GREY_90: f64 = 0.5675779119928026;
const PQ_REF_WHITE: f64 = 0.5806888810416109;
const SDR_DIFF: f64 = 0.0001;
const SDR_BLACK: f64 = 0.0;
const SDR_GREY_18: f64 = 0.3870226424119708;
const SDR_GREY_83: f64 = 0.8294599123812822;
const SDR_GREY_90: f64 = 0.8646750115119883;
const SDR_REF_WHITE: f64 = 0.9112149320796772;

#[test]
fn test_map_rw_100_peak_492() {

    let mut frame = vec![
        Pixel { red: PQ_BLACK, green: PQ_BLACK, blue: PQ_BLACK },
        Pixel { red: PQ_100_NITS, green: PQ_100_NITS, blue: PQ_100_NITS },
        Pixel { red: PQ_492_NITS, green: PQ_492_NITS, blue: PQ_492_NITS },
        Pixel { red: PQ_4000_NITS, green: PQ_4000_NITS, blue: PQ_4000_NITS },
        Pixel { red: PQ_10000_NITS, green: PQ_10000_NITS, blue: PQ_10000_NITS },
    ];
    let pq_hlg_mapper =
        PqHlgMapper::new_by_ref_white(100.0, 0.49261083743842365, ToneMapMethod::MaxRgb);

    for pixel in frame.iter_mut() {
        *pixel = pq_hlg_mapper.map(*pixel);
    }

    assert_approx_eq!(frame[0].red, 0.0, HDR_DIFF);
    assert_approx_eq!(frame[0].green, 0.0, HDR_DIFF);
    assert_approx_eq!(frame[0].blue, 0.0, HDR_DIFF);

    assert_approx_eq!(frame[1].red, 0.7498773, HDR_DIFF);
    assert_approx_eq!(frame[1].green, 0.7498773, HDR_DIFF);
    assert_approx_eq!(frame[1].blue, 0.7498773, HDR_DIFF);

    assert_approx_eq!(frame[2].red, 1.0, HDR_DIFF);
    assert_approx_eq!(frame[2].green, 1.0, HDR_DIFF);
    assert_approx_eq!(frame[2].blue, 1.0, HDR_DIFF);

    assert_gt!(frame[3].red, 1.0);
    assert_gt!(frame[3].green, 1.0);
    assert_gt!(frame[3].blue, 1.0);

    assert_gt!(frame[4].red, 1.0);
    assert_gt!(frame[4].green, 1.0);
    assert_gt!(frame[4].blue, 1.0);
}

#[test]
fn test_map_rw_100_peak_1_970() {

    let mut frame = vec![
        Pixel { red: PQ_BLACK, green: PQ_BLACK, blue: PQ_BLACK },
        Pixel { red: PQ_100_NITS, green: PQ_100_NITS, blue: PQ_100_NITS },
        Pixel { red: PQ_1970_NITS, green: PQ_1970_NITS, blue: PQ_1970_NITS },
        Pixel { red: PQ_4000_NITS, green: PQ_4000_NITS, blue: PQ_4000_NITS },
        Pixel { red: PQ_10000_NITS, green: PQ_10000_NITS, blue: PQ_10000_NITS },
    ];
    let pq_hlg_mapper =
        PqHlgMapper::new_by_ref_white(100.0, 1970.4433497536945, ToneMapMethod::MaxRgb);

    for pixel in frame.iter_mut() {
        *pixel = pq_hlg_mapper.map(*pixel);
    }

    assert_approx_eq!(frame[0].red, 0.0, HDR_DIFF);
    assert_approx_eq!(frame[0].green, 0.0, HDR_DIFF);
    assert_approx_eq!(frame[0].blue, 0.0, HDR_DIFF);

    assert_approx_eq!(frame[1].red, 0.7498773, HDR_DIFF);
    assert_approx_eq!(frame[1].green, 0.7498773, HDR_DIFF);
    assert_approx_eq!(frame[1].blue, 0.7498773, HDR_DIFF);

    assert_approx_eq!(frame[2].red, 1.0, HDR_DIFF);
    assert_approx_eq!(frame[2].green, 1.0, HDR_DIFF);
    assert_approx_eq!(frame[2].blue, 1.0, HDR_DIFF);

    assert_gt!(frame[3].red, 1.0);
    assert_gt!(frame[3].green, 1.0);
    assert_gt!(frame[3].blue, 1.0);

    assert_gt!(frame[4].red, 1.0);
    assert_gt!(frame[4].green, 1.0);
    assert_gt!(frame[4].blue, 1.0);
}

#[test]
fn test_map_rw_100_peak_4_926() {

    let mut frame = vec![
        Pixel { red: PQ_BLACK, green: PQ_BLACK, blue: PQ_BLACK },
        Pixel { red: PQ_100_NITS, green: PQ_100_NITS, blue: PQ_100_NITS },
        Pixel { red: PQ_4926_NITS, green: PQ_4926_NITS, blue: PQ_4926_NITS },
        Pixel { red: PQ_10000_NITS, green: PQ_10000_NITS, blue: PQ_10000_NITS },
    ];
    let pq_hlg_mapper = PqHlgMapper::new_by_ref_white(100.0, 4_926.0, ToneMapMethod::MaxRgb);

    for pixel in frame.iter_mut() {
        *pixel = pq_hlg_mapper.map(*pixel);
    }

    assert_approx_eq!(frame[0].red, 0.0, HDR_DIFF);
    assert_approx_eq!(frame[0].green, 0.0, HDR_DIFF);
    assert_approx_eq!(frame[0].blue, 0.0, HDR_DIFF);

    assert_approx_eq!(frame[1].red, 0.7498773, HDR_DIFF);
    assert_approx_eq!(frame[1].green, 0.7498773, HDR_DIFF);
    assert_approx_eq!(frame[1].blue, 0.7498773, HDR_DIFF);

    assert_approx_eq!(frame[2].red, 1.0, HDR_DIFF);
    assert_approx_eq!(frame[2].green, 1.0, HDR_DIFF);
    assert_approx_eq!(frame[2].blue, 1.0, HDR_DIFF);

    assert_gt!(frame[3].red, 1.0);
    assert_gt!(frame[3].green, 1.0);
    assert_gt!(frame[3].blue, 1.0);
}

#[test]
fn test_map_rw_203_peak_500() {

    let mut frame = vec![
        Pixel { red: PQ_BLACK, green: PQ_BLACK, blue: PQ_BLACK },
        Pixel { red: PQ_REF_WHITE, green: PQ_REF_WHITE, blue: PQ_REF_WHITE },
        Pixel { red: PQ_1000_NITS, green: PQ_1000_NITS, blue: PQ_1000_NITS },
        Pixel { red: PQ_4000_NITS, green: PQ_4000_NITS, blue: PQ_4000_NITS },
        Pixel { red: PQ_10000_NITS, green: PQ_10000_NITS, blue: PQ_10000_NITS },
    ];
    let pq_hlg_mapper = PqHlgMapper::new_by_ref_white(203.0, 500.0, ToneMapMethod::MaxRgb);

    for pixel in frame.iter_mut() {
        *pixel = pq_hlg_mapper.map(*pixel);
    }

    assert_approx_eq!(frame[0].red, 0.0, HDR_DIFF);
    assert_approx_eq!(frame[0].green, 0.0, HDR_DIFF);
    assert_approx_eq!(frame[0].blue, 0.0, HDR_DIFF);

    assert_approx_eq!(frame[1].red, 0.7498773, HDR_DIFF);
    assert_approx_eq!(frame[1].green, 0.7498773, HDR_DIFF);
    assert_approx_eq!(frame[1].blue, 0.7498773, HDR_DIFF);

    assert_approx_eq!(frame[2].red, 1.0, HDR_DIFF);
    assert_approx_eq!(frame[2].green, 1.0, HDR_DIFF);
    assert_approx_eq!(frame[2].blue, 1.0, HDR_DIFF);

    assert_gt!(frame[3].red, 1.0);
    assert_gt!(frame[3].green, 1.0);
    assert_gt!(frame[3].blue, 1.0);

    assert_gt!(frame[4].red, 1.0);
    assert_gt!(frame[4].green, 1.0);
    assert_gt!(frame[4].blue, 1.0);
}

#[test]
fn test_map_rw_203_peak_1_000() {

    let mut frame = vec![
        Pixel { red: PQ_BLACK, green: PQ_BLACK, blue: PQ_BLACK },
        Pixel { red: PQ_REF_WHITE, green: PQ_REF_WHITE, blue: PQ_REF_WHITE },
        Pixel { red: PQ_1000_NITS, green: PQ_1000_NITS, blue: PQ_1000_NITS },
        Pixel { red: PQ_4000_NITS, green: PQ_4000_NITS, blue: PQ_4000_NITS },
        Pixel { red: PQ_10000_NITS, green: PQ_10000_NITS, blue: PQ_10000_NITS },
    ];
    let pq_hlg_mapper = PqHlgMapper::new_by_ref_white(203.0, 1_000.0, ToneMapMethod::MaxRgb);

    for pixel in frame.iter_mut() {
        *pixel = pq_hlg_mapper.map(*pixel);
    }

    assert_approx_eq!(frame[0].red, 0.0, HDR_DIFF);
    assert_approx_eq!(frame[0].green, 0.0, HDR_DIFF);
    assert_approx_eq!(frame[0].blue, 0.0, HDR_DIFF);

    assert_approx_eq!(frame[1].red, 0.7498773, HDR_DIFF);
    assert_approx_eq!(frame[1].green, 0.7498773, HDR_DIFF);
    assert_approx_eq!(frame[1].blue, 0.7498773, HDR_DIFF);

    assert_approx_eq!(frame[2].red, 1.0, HDR_DIFF);
    assert_approx_eq!(frame[2].green, 1.0, HDR_DIFF);
    assert_approx_eq!(frame[2].blue, 1.0, HDR_DIFF);

    assert_gt!(frame[3].red, 1.0);
    assert_gt!(frame[3].green, 1.0);
    assert_gt!(frame[3].blue, 1.0);

    assert_gt!(frame[4].red, 1.0);
    assert_gt!(frame[4].green, 1.0);
    assert_gt!(frame[4].blue, 1.0);
}

#[test]
fn test_map_rw_203_peak_4_000() {

    let mut frame = vec![
        Pixel { red: PQ_BLACK, green: PQ_BLACK, blue: PQ_BLACK },
        Pixel { red: PQ_REF_WHITE, green: PQ_REF_WHITE, blue: PQ_REF_WHITE },
        Pixel { red: PQ_4000_NITS, green: PQ_4000_NITS, blue: PQ_4000_NITS },
        Pixel { red: PQ_10000_NITS, green: PQ_10000_NITS, blue: PQ_10000_NITS },
    ];
    let pq_hlg_mapper = PqHlgMapper::new_by_ref_white(203.0, 4_000.0, ToneMapMethod::MaxRgb);

    for pixel in frame.iter_mut() {
        *pixel = pq_hlg_mapper.map(*pixel);
    }

    assert_approx_eq!(frame[0].red, 0.0, HDR_DIFF);
    assert_approx_eq!(frame[0].green, 0.0, HDR_DIFF);
    assert_approx_eq!(frame[0].blue, 0.0, HDR_DIFF);

    assert_approx_eq!(frame[1].red, 0.7498773, HDR_DIFF);
    assert_approx_eq!(frame[1].green, 0.7498773, HDR_DIFF);
    assert_approx_eq!(frame[1].blue, 0.7498773, HDR_DIFF);

    assert_approx_eq!(frame[2].red, 1.0, HDR_DIFF);
    assert_approx_eq!(frame[2].green, 1.0, HDR_DIFF);
    assert_approx_eq!(frame[2].blue, 1.0, HDR_DIFF);

    assert_gt!(frame[3].red, 1.0);
    assert_gt!(frame[3].green, 1.0);
    assert_gt!(frame[3].blue, 1.0);
}

#[test]
fn test_map_rw_203_peak_10_000() {

    let mut frame = vec![
        Pixel { red: PQ_BLACK, green: PQ_BLACK, blue: PQ_BLACK },
        Pixel { red: PQ_REF_WHITE, green: PQ_REF_WHITE, blue: PQ_REF_WHITE },
        Pixel { red: PQ_10000_NITS, green: PQ_10000_NITS, blue: PQ_10000_NITS },
    ];
    let pq_hlg_mapper = PqHlgMapper::new_by_ref_white(203.0, 10_000.0, ToneMapMethod::MaxRgb);

    for pixel in frame.iter_mut() {
        *pixel = pq_hlg_mapper.map(*pixel);
    }

    assert_approx_eq!(frame[0].red, 0.0, HDR_DIFF);
    assert_approx_eq!(frame[0].green, 0.0, HDR_DIFF);
    assert_approx_eq!(frame[0].blue, 0.0, HDR_DIFF);

    assert_approx_eq!(frame[1].red, 0.7498773, HDR_DIFF);
    assert_approx_eq!(frame[1].green, 0.7498773, HDR_DIFF);
    assert_approx_eq!(frame[1].blue, 0.7498773, HDR_DIFF);

    assert_approx_eq!(frame[2].red, 1.0, HDR_DIFF);
    assert_approx_eq!(frame[2].green, 1.0, HDR_DIFF);
    assert_approx_eq!(frame[2].blue, 1.0, HDR_DIFF);
}

#[test]
fn test_preview_map() {

    let mut frame = vec![
        Pixel { red: PQ_BLACK, green: PQ_BLACK, blue: PQ_BLACK },
        Pixel { red: PQ_GREY_18, green: PQ_GREY_18, blue: PQ_GREY_18 },
        Pixel { red: PQ_GREY_83, green: PQ_GREY_83, blue: PQ_GREY_83 },
        Pixel { red: PQ_GREY_90, green: PQ_GREY_90, blue: PQ_GREY_90 },
        Pixel { red: PQ_REF_WHITE, green: PQ_REF_WHITE, blue: PQ_REF_WHITE },
    ];
    let pq_hlg_mapper = PqSdrMapper::new_by_factor(1.0, 10_000.0, ToneMapMethod::MaxRgb);

    for pixel in frame.iter_mut() {
        *pixel = pq_hlg_mapper.map(*pixel);
    }

    assert_approx_eq!(frame[0].red, SDR_BLACK, SDR_DIFF);
    assert_approx_eq!(frame[0].green, SDR_BLACK, SDR_DIFF);
    assert_approx_eq!(frame[0].blue, SDR_BLACK, SDR_DIFF);

    assert_approx_eq!(frame[1].red, SDR_GREY_18, SDR_DIFF);
    assert_approx_eq!(frame[1].green, SDR_GREY_18, SDR_DIFF);
    assert_approx_eq!(frame[1].blue, SDR_GREY_18, SDR_DIFF);

    assert_approx_eq!(frame[2].red, SDR_GREY_83, SDR_DIFF);
    assert_approx_eq!(frame[2].green, SDR_GREY_83, SDR_DIFF);
    assert_approx_eq!(frame[2].blue, SDR_GREY_83, SDR_DIFF);

    assert_approx_eq!(frame[3].red, SDR_GREY_90, SDR_DIFF);
    assert_approx_eq!(frame[3].green, SDR_GREY_90, SDR_DIFF);
    assert_approx_eq!(frame[3].blue, SDR_GREY_90, SDR_DIFF);

    assert_approx_eq!(frame[4].red, SDR_REF_WHITE, SDR_DIFF);
    assert_approx_eq!(frame[4].green, SDR_REF_WHITE, SDR_DIFF);
    assert_approx_eq!(frame[4].blue, SDR_REF_WHITE, SDR_DIFF);
}
