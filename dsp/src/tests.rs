/*
 * SPDX-FileCopyrightText: 2021 William Swartzendruber <wswartzendruber@gmail.com>
 *
 * SPDX-License-Identifier: CC0-1.0
 */

use super::*;

use assert_approx_eq::assert_approx_eq;

const DIFF: f64 = 0.0000001;
const PQ_1000_NITS: f64 = 0.751827096247041;
const PQ_4000_NITS: f64 = 0.9025723933109373;
const PQ_10000_NITS: f64 = 1.0;
const PQ_BLACK: f64 = 0.0;
const PQ_REF_WHITE: f64 = 0.5806888810416109;

#[test]
fn test_map_rw_203_peak_500() {

    let mut frame = vec![
        RgbPixel { red: PQ_BLACK, green: PQ_BLACK, blue: PQ_BLACK },
        RgbPixel { red: PQ_REF_WHITE, green: PQ_REF_WHITE, blue: PQ_REF_WHITE },
        RgbPixel { red: PQ_1000_NITS, green: PQ_1000_NITS, blue: PQ_1000_NITS },
        RgbPixel { red: PQ_4000_NITS, green: PQ_4000_NITS, blue: PQ_4000_NITS },
        RgbPixel { red: PQ_10000_NITS, green: PQ_10000_NITS, blue: PQ_10000_NITS },
    ];
    let pq_hlg_mapper = PqHlgMapper::new(500.0, 1.0);

    for pixel in frame.iter_mut() {
        *pixel = pq_hlg_mapper.map(*pixel);
    }

    assert_approx_eq!(frame[0].red, 0.0, DIFF);
    assert_approx_eq!(frame[0].green, 0.0, DIFF);
    assert_approx_eq!(frame[0].blue, 0.0, DIFF);

    assert_approx_eq!(frame[1].red, 0.7498773, DIFF);
    assert_approx_eq!(frame[1].green, 0.7498773, DIFF);
    assert_approx_eq!(frame[1].blue, 0.7498773, DIFF);

    assert_approx_eq!(frame[2].red, 1.0, DIFF);
    assert_approx_eq!(frame[2].green, 1.0, DIFF);
    assert_approx_eq!(frame[2].blue, 1.0, DIFF);

    assert_approx_eq!(frame[3].red, 1.0, DIFF);
    assert_approx_eq!(frame[3].green, 1.0, DIFF);
    assert_approx_eq!(frame[3].blue, 1.0, DIFF);

    assert_approx_eq!(frame[4].red, 1.0, DIFF);
    assert_approx_eq!(frame[4].green, 1.0, DIFF);
    assert_approx_eq!(frame[4].blue, 1.0, DIFF);
}

#[test]
fn test_map_rw_203_peak_1_000() {

    let mut frame = vec![
        RgbPixel { red: PQ_BLACK, green: PQ_BLACK, blue: PQ_BLACK },
        RgbPixel { red: PQ_REF_WHITE, green: PQ_REF_WHITE, blue: PQ_REF_WHITE },
        RgbPixel { red: PQ_1000_NITS, green: PQ_1000_NITS, blue: PQ_1000_NITS },
        RgbPixel { red: PQ_4000_NITS, green: PQ_4000_NITS, blue: PQ_4000_NITS },
        RgbPixel { red: PQ_10000_NITS, green: PQ_10000_NITS, blue: PQ_10000_NITS },
    ];
    let pq_hlg_mapper = PqHlgMapper::new(1_000.0, 1.0);

    for pixel in frame.iter_mut() {
        *pixel = pq_hlg_mapper.map(*pixel);
    }

    assert_approx_eq!(frame[0].red, 0.0, DIFF);
    assert_approx_eq!(frame[0].green, 0.0, DIFF);
    assert_approx_eq!(frame[0].blue, 0.0, DIFF);

    assert_approx_eq!(frame[1].red, 0.7498773, DIFF);
    assert_approx_eq!(frame[1].green, 0.7498773, DIFF);
    assert_approx_eq!(frame[1].blue, 0.7498773, DIFF);

    assert_approx_eq!(frame[2].red, 1.0, DIFF);
    assert_approx_eq!(frame[2].green, 1.0, DIFF);
    assert_approx_eq!(frame[2].blue, 1.0, DIFF);

    assert_approx_eq!(frame[3].red, 1.0, DIFF);
    assert_approx_eq!(frame[3].green, 1.0, DIFF);
    assert_approx_eq!(frame[3].blue, 1.0, DIFF);

    assert_approx_eq!(frame[4].red, 1.0, DIFF);
    assert_approx_eq!(frame[4].green, 1.0, DIFF);
    assert_approx_eq!(frame[4].blue, 1.0, DIFF);
}

#[test]
fn test_map_rw_203_peak_4_000() {

    let mut frame = vec![
        RgbPixel { red: PQ_BLACK, green: PQ_BLACK, blue: PQ_BLACK },
        RgbPixel { red: PQ_REF_WHITE, green: PQ_REF_WHITE, blue: PQ_REF_WHITE },
        RgbPixel { red: PQ_1000_NITS, green: PQ_1000_NITS, blue: PQ_1000_NITS },
        RgbPixel { red: PQ_4000_NITS, green: PQ_4000_NITS, blue: PQ_4000_NITS },
        RgbPixel { red: PQ_10000_NITS, green: PQ_10000_NITS, blue: PQ_10000_NITS },
    ];
    let pq_hlg_mapper = PqHlgMapper::new(4_000.0, 1.0);

    for pixel in frame.iter_mut() {
        *pixel = pq_hlg_mapper.map(*pixel);
    }

    assert_approx_eq!(frame[0].red, 0.0, DIFF);
    assert_approx_eq!(frame[0].green, 0.0, DIFF);
    assert_approx_eq!(frame[0].blue, 0.0, DIFF);

    assert_approx_eq!(frame[1].red, 0.7498773, DIFF);
    assert_approx_eq!(frame[1].green, 0.7498773, DIFF);
    assert_approx_eq!(frame[1].blue, 0.7498773, DIFF);

    assert!(frame[2].red < 1.0);
    assert!(frame[2].green < 1.0);
    assert!(frame[2].blue < 1.0);

    assert_approx_eq!(frame[3].red, 1.0, DIFF);
    assert_approx_eq!(frame[3].green, 1.0, DIFF);
    assert_approx_eq!(frame[3].blue, 1.0, DIFF);

    assert_approx_eq!(frame[4].red, 1.0, DIFF);
    assert_approx_eq!(frame[4].green, 1.0, DIFF);
    assert_approx_eq!(frame[4].blue, 1.0, DIFF);
}

#[test]
fn test_map_rw_203_peak_10_000() {

    let mut frame = vec![
        RgbPixel { red: PQ_BLACK, green: PQ_BLACK, blue: PQ_BLACK },
        RgbPixel { red: PQ_REF_WHITE, green: PQ_REF_WHITE, blue: PQ_REF_WHITE },
        RgbPixel { red: PQ_1000_NITS, green: PQ_1000_NITS, blue: PQ_1000_NITS },
        RgbPixel { red: PQ_4000_NITS, green: PQ_4000_NITS, blue: PQ_4000_NITS },
        RgbPixel { red: PQ_10000_NITS, green: PQ_10000_NITS, blue: PQ_10000_NITS },
    ];
    let pq_hlg_mapper = PqHlgMapper::new(10_000.0, 1.0);

    for pixel in frame.iter_mut() {
        *pixel = pq_hlg_mapper.map(*pixel);
    }

    assert_approx_eq!(frame[0].red, 0.0, DIFF);
    assert_approx_eq!(frame[0].green, 0.0, DIFF);
    assert_approx_eq!(frame[0].blue, 0.0, DIFF);

    assert_approx_eq!(frame[1].red, 0.7498773, DIFF);
    assert_approx_eq!(frame[1].green, 0.7498773, DIFF);
    assert_approx_eq!(frame[1].blue, 0.7498773, DIFF);

    assert!(frame[2].red < frame[3].red);
    assert!(frame[2].green < frame[3].green);
    assert!(frame[2].blue < frame[3].blue);

    assert!(frame[3].red < 1.0);
    assert!(frame[3].green < 1.0);
    assert!(frame[3].blue < 1.0);

    assert_approx_eq!(frame[4].red, 1.0, DIFF);
    assert_approx_eq!(frame[4].green, 1.0, DIFF);
    assert_approx_eq!(frame[4].blue, 1.0, DIFF);
}
