/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

use super::*;

use assert_approx_eq::assert_approx_eq;

const DIFF: f64 = 0.0000001;
const PQ_0_NITS: f64 = 0.0;
const PQ_203_NITS: f64 = 0.5806888810416109;
const PQ_1000_NITS: f64 = 0.751827096247041;
const PQ_4000_NITS: f64 = 0.9025723933109373;
const PQ_10000_NITS: f64 = 1.0;

#[test]
fn test_map_peak_500() {

    let mut frame = vec![
        Pixel { red: PQ_0_NITS, green: PQ_0_NITS, blue: PQ_0_NITS },
        Pixel { red: PQ_203_NITS, green: PQ_203_NITS, blue: PQ_203_NITS },
        Pixel { red: PQ_1000_NITS, green: PQ_1000_NITS, blue: PQ_1000_NITS },
        Pixel { red: PQ_4000_NITS, green: PQ_4000_NITS, blue: PQ_4000_NITS },
        Pixel { red: PQ_10000_NITS, green: PQ_10000_NITS, blue: PQ_10000_NITS },
    ];
    let pq_hlg_mapper = PqHlgMapper::new(203.0, 500.0);

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
fn test_map_peak_1_000() {

    let mut frame = vec![
        Pixel { red: PQ_0_NITS, green: PQ_0_NITS, blue: PQ_0_NITS },
        Pixel { red: PQ_203_NITS, green: PQ_203_NITS, blue: PQ_203_NITS },
        Pixel { red: PQ_1000_NITS, green: PQ_1000_NITS, blue: PQ_1000_NITS },
        Pixel { red: PQ_4000_NITS, green: PQ_4000_NITS, blue: PQ_4000_NITS },
        Pixel { red: PQ_10000_NITS, green: PQ_10000_NITS, blue: PQ_10000_NITS },
    ];
    let pq_hlg_mapper = PqHlgMapper::new(203.0, 1_000.0);

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
fn test_map_peak_4_000() {

    let mut frame = vec![
        Pixel { red: PQ_0_NITS, green: PQ_0_NITS, blue: PQ_0_NITS },
        Pixel { red: PQ_203_NITS, green: PQ_203_NITS, blue: PQ_203_NITS },
        Pixel { red: PQ_1000_NITS, green: PQ_1000_NITS, blue: PQ_1000_NITS },
        Pixel { red: PQ_4000_NITS, green: PQ_4000_NITS, blue: PQ_4000_NITS },
        Pixel { red: PQ_10000_NITS, green: PQ_10000_NITS, blue: PQ_10000_NITS },
    ];
    let pq_hlg_mapper = PqHlgMapper::new(203.0, 4_000.0);

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
fn test_map_peak_10_000() {

    let mut frame = vec![
        Pixel { red: PQ_0_NITS, green: PQ_0_NITS, blue: PQ_0_NITS },
        Pixel { red: PQ_203_NITS, green: PQ_203_NITS, blue: PQ_203_NITS },
        Pixel { red: PQ_1000_NITS, green: PQ_1000_NITS, blue: PQ_1000_NITS },
        Pixel { red: PQ_4000_NITS, green: PQ_4000_NITS, blue: PQ_4000_NITS },
        Pixel { red: PQ_10000_NITS, green: PQ_10000_NITS, blue: PQ_10000_NITS },
    ];
    let pq_hlg_mapper = PqHlgMapper::new(203.0, 10_000.0);

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
