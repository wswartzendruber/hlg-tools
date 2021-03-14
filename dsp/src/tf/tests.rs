/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

use super::*;

use assert_approx_eq::assert_approx_eq;

const EOTF_DIFF: f64 = 0.0005;     // ±5 nits
const OETF_DIFF: f64 = 0.005;      // +0.5%
const OOTF_DIFF: f64 = 0.00000001; //

#[test]
fn test_pq_eotf() {

    //
    // SMPTE Presentation
    // "HDR (High Dynamic Range)"
    // "Technical Considerations for Live Production and Distribution Workflows"
    // Hugo Gaggioni
    // 2018-05-21
    // Slide #28
    //

    // MINIMUM: A 0% signal level shall match 0 nits.
    assert_approx_eq!(pq_eotf(0.0), 0.0, EOTF_DIFF);

    // 18% WHITE CARD: A 38% signal level shall match 26 nits.
    assert_approx_eq!(pq_eotf(0.38), 0.0026, EOTF_DIFF);

    // 83% WHITE CARD: A 56% signal level shall match 162 nits.
    assert_approx_eq!(pq_eotf(0.56), 0.0162, EOTF_DIFF);

    // 90% WHITE CARD: A 57% signal level shall match 179 nits.
    assert_approx_eq!(pq_eotf(0.57), 0.0179, EOTF_DIFF);

    // 100% WHITE CARD: A 58% signal level shall match 203 nits.
    assert_approx_eq!(pq_eotf(0.58), 0.0203, EOTF_DIFF);

    // MAXIMUM: A 100% signal level shall match 10,000 nits.
    assert_approx_eq!(pq_eotf(1.0), 1.0, EOTF_DIFF);
}

#[test]
fn test_pq_oetf() {

    //
    // SMPTE Presentation
    // "HDR (High Dynamic Range)"
    // "Technical Considerations for Live Production and Distribution Workflows"
    // Hugo Gaggioni
    // 2018-05-21
    // Slide #28
    //

    // MINIMUM: 0 nits shall match a 0% signal level.
    assert_approx_eq!(pq_oetf(0.0), 0.0, OETF_DIFF);

    // 18% WHITE CARD: 26 nits shall match a 38% signal level.
    assert_approx_eq!(pq_oetf(0.0026), 0.38, OETF_DIFF);

    // 83% WHITE CARD: 162 nits shall match a 56% signal level.
    assert_approx_eq!(pq_oetf(0.0162), 0.56, OETF_DIFF);

    // 90% WHITE CARD: 179 nits shall match a 57% signal level.
    assert_approx_eq!(pq_oetf(0.0179), 0.57, OETF_DIFF);

    // 100% WHITE CARD: 203 nits shall match a 58% signal level.
    assert_approx_eq!(pq_oetf(0.0203), 0.58, OETF_DIFF);

    // MAXIMUM: 10,000 nits shall match a 1000% signal level.
    assert_approx_eq!(pq_oetf(1.0), 1.0, OETF_DIFF);
}

#[test]
fn test_hlg_oetf() {

    //
    // LOWER CURVE
    //

    assert_approx_eq!(hlg_oetf(1.0 / 96.0), 0.176, OETF_DIFF);
    assert_approx_eq!(hlg_oetf(2.0 / 96.0), 0.250, OETF_DIFF);
    assert_approx_eq!(hlg_oetf(3.0 / 96.0), 0.306, OETF_DIFF);
    assert_approx_eq!(hlg_oetf(4.0 / 96.0), 0.353, OETF_DIFF);
    assert_approx_eq!(hlg_oetf(5.0 / 96.0), 0.395, OETF_DIFF);
    assert_approx_eq!(hlg_oetf(6.0 / 96.0), 0.433, OETF_DIFF);
    assert_approx_eq!(hlg_oetf(7.0 / 96.0), 0.467, OETF_DIFF);

    //
    // INTERSECTION POINT
    //

    assert_approx_eq!(hlg_oetf(1.0 / 12.0), 0.500, OETF_DIFF);

    //
    // UPPER CURVE
    //

    assert_approx_eq!(hlg_oetf(2.0 / 12.0), 0.656, OETF_DIFF);
    assert_approx_eq!(hlg_oetf(3.0 / 12.0), 0.738, OETF_DIFF);
    assert_approx_eq!(hlg_oetf(4.0 / 12.0), 0.794, OETF_DIFF);
    assert_approx_eq!(hlg_oetf(5.0 / 12.0), 0.837, OETF_DIFF);
    assert_approx_eq!(hlg_oetf(6.0 / 12.0), 0.871, OETF_DIFF);
    assert_approx_eq!(hlg_oetf(7.0 / 12.0), 0.900, OETF_DIFF);
    assert_approx_eq!(hlg_oetf(8.0 / 12.0), 0.925, OETF_DIFF);
    assert_approx_eq!(hlg_oetf(9.0 / 12.0), 0.947, OETF_DIFF);
    assert_approx_eq!(hlg_oetf(10.0 / 12.0), 0.966, OETF_DIFF);
    assert_approx_eq!(hlg_oetf(11.0 / 12.0), 0.984, OETF_DIFF);
    assert_approx_eq!(hlg_oetf(12.0 / 12.0), 0.999, OETF_DIFF);
}

#[test]
fn test_hlg_iootf() {

    let pq_pixel_000 = Pixel { red: 0.0, green: 0.0, blue: 0.0 };
    let hlg_pixel_000 = hlg_iootf(pq_pixel_000);

    assert_approx_eq!(hlg_pixel_000.red, 0.0, OOTF_DIFF);
    assert_approx_eq!(hlg_pixel_000.green, 0.0, OOTF_DIFF);
    assert_approx_eq!(hlg_pixel_000.blue, 0.0, OOTF_DIFF);

    let pq_pixel_001 = Pixel { red: 0.0, green: 0.0, blue: 0.1 };
    let hlg_pixel_001 = hlg_iootf(pq_pixel_001);

    assert_approx_eq!(hlg_pixel_001.red, 0.0, OOTF_DIFF);
    assert_approx_eq!(hlg_pixel_001.green, 0.0, OOTF_DIFF);
    assert_approx_eq!(hlg_pixel_001.blue, 1.60136703633, OOTF_DIFF);

    let pq_pixel_010 = Pixel { red: 0.0, green: 0.1, blue: 0.0 };
    let hlg_pixel_010 = hlg_iootf(pq_pixel_010);

    assert_approx_eq!(hlg_pixel_010.red, 0.0, OOTF_DIFF);
    assert_approx_eq!(hlg_pixel_010.green, 1.06691147061, OOTF_DIFF);
    assert_approx_eq!(hlg_pixel_010.blue, 0.0, OOTF_DIFF);

    let pq_pixel_011 = Pixel { red: 0.0, green: 0.1, blue: 0.1 };
    let hlg_pixel_011 = hlg_iootf(pq_pixel_011);

    assert_approx_eq!(hlg_pixel_011.red, 0.0, OOTF_DIFF);
    assert_approx_eq!(hlg_pixel_011.green, 1.05210550828, OOTF_DIFF);
    assert_approx_eq!(hlg_pixel_011.blue, 1.05210550828, OOTF_DIFF);

    let pq_pixel_100 = Pixel { red: 0.1, green: 0.0, blue: 0.0 };
    let hlg_pixel_100 = hlg_iootf(pq_pixel_100);

    assert_approx_eq!(hlg_pixel_100.red, 1.24955867676, OOTF_DIFF);
    assert_approx_eq!(hlg_pixel_100.green, 0.0, OOTF_DIFF);
    assert_approx_eq!(hlg_pixel_100.blue, 0.0, OOTF_DIFF);

    let pq_pixel_101 = Pixel { red: 0.1, green: 0.0, blue: 0.1 };
    let hlg_pixel_101 = hlg_iootf(pq_pixel_101);

    assert_approx_eq!(hlg_pixel_101.red, 1.20788064268, OOTF_DIFF);
    assert_approx_eq!(hlg_pixel_101.green, 0.0, OOTF_DIFF);
    assert_approx_eq!(hlg_pixel_101.blue, 1.20788064268, OOTF_DIFF);

    let pq_pixel_110 = Pixel { red: 0.1, green: 0.1, blue: 0.0 };
    let hlg_pixel_110 = hlg_iootf(pq_pixel_110);

    assert_approx_eq!(hlg_pixel_110.red, 1.01024057949, OOTF_DIFF);
    assert_approx_eq!(hlg_pixel_110.green, 1.01024057949, OOTF_DIFF);
    assert_approx_eq!(hlg_pixel_110.blue, 0.0, OOTF_DIFF);

    let pq_pixel_111 = Pixel { red: 0.1, green: 0.1, blue: 0.1 };
    let hlg_pixel_111 = hlg_iootf(pq_pixel_111);

    assert_approx_eq!(hlg_pixel_111.red, 1.0, OOTF_DIFF);
    assert_approx_eq!(hlg_pixel_111.green, 1.0, OOTF_DIFF);
    assert_approx_eq!(hlg_pixel_111.blue, 1.0, OOTF_DIFF);
}
