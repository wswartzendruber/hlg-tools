/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

use super::*;

use assert_approx_eq::assert_approx_eq;

const DIFF: f64 = 0.01;

#[test]
fn test_map_peak_1_000() {

    let mut frame = vec![
        Pixel { red: 0.00, green: 0.00, blue: 0.00 },
        Pixel { red: 0.58, green: 0.58, blue: 0.58 },
        Pixel { red: 0.75, green: 0.75, blue: 0.75 },
        Pixel { red: 1.00, green: 1.00, blue: 1.00 },
    ];
    let pq_hlg_mapper = PqHlgMapper::new(203.0, 1_000.0);

    for pixel in frame.iter_mut() {
        *pixel = pq_hlg_mapper.map(*pixel);
    }

    assert_approx_eq!(frame[0].red, 0.0, DIFF);
    assert_approx_eq!(frame[0].green, 0.0, DIFF);
    assert_approx_eq!(frame[0].blue, 0.0, DIFF);

    assert_approx_eq!(frame[1].red, 0.75, DIFF);
    assert_approx_eq!(frame[1].green, 0.75, DIFF);
    assert_approx_eq!(frame[1].blue, 0.75, DIFF);

    assert_approx_eq!(frame[2].red, 0.99, DIFF);
    assert_approx_eq!(frame[2].green, 0.99, DIFF);
    assert_approx_eq!(frame[2].blue, 0.99, DIFF);

    assert_eq!(frame[3], Pixel {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    });
}
