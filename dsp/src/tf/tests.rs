/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

use super::*;

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
    assert_eq!((pq_eotf(0.0000) * 10000.0).round() as u16, 0);

    // 18% WHITE CARD: A 38% signal level shall match 26 nits.
    assert_eq!((pq_eotf(0.3800) * 10000.0).round() as u16, 26);

    // 83% WHITE CARD: A 56% signal level shall match 162 nits.
    assert_eq!((pq_eotf(0.5575) * 10000.0).round() as u16, 162);

    // 90% WHITE CARD: A 57% signal level shall match 179 nits.
    assert_eq!((pq_eotf(0.5675) * 10000.0).round() as u16, 179);

    // 100% WHITE CARD: A 58% signal level shall match 203 nits.
    assert_eq!((pq_eotf(0.5805) * 10000.0).round() as u16, 203);

    // MAXIMUM: A 100% signal level shall match 10,000 nits.
    assert_eq!((pq_eotf(1.0000) * 10000.0).round() as u16, 10000);
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

    // MINIMUM: A 0% signal level shall match 0 nits.
    assert_eq!((pq_oetf(0.0 / 10_000.0) * 100.0).round() as u8, 0);

    // 18% WHITE CARD: A 38% signal level shall match 26 nits.
    assert_eq!((pq_oetf(26.0 / 10_000.0) * 100.0).round() as u8, 38);

    // 83% WHITE CARD: A 56% signal level shall match 162 nits.
    assert_eq!((pq_oetf(162.0 / 10_000.0) * 100.0).round() as u8, 56);

    // 90% WHITE CARD: A 57% signal level shall match 179 nits.
    assert_eq!((pq_oetf(179.0 / 10_000.0) * 100.0).round() as u8, 57);

    // 100% WHITE CARD: A 58% signal level shall match 203 nits.
    assert_eq!((pq_oetf(203.0 / 10_000.0) * 100.0).round() as u8, 58);

    // MAXIMUM: A 100% signal level shall match 10,000 nits.
    assert_eq!((pq_oetf(10_000.0 / 10_000.0) * 100.0).round() as u8, 100);
}

#[test]
fn test_hlg_oetf() {

    //
    // LOWER CURVE
    //

    assert_eq!((hlg_oetf(1.0 / 96.0) * 1000.0) as u16, 176);
    assert_eq!((hlg_oetf(2.0 / 96.0) * 1000.0) as u16, 250);
    assert_eq!((hlg_oetf(3.0 / 96.0) * 1000.0) as u16, 306);
    assert_eq!((hlg_oetf(4.0 / 96.0) * 1000.0) as u16, 353);
    assert_eq!((hlg_oetf(5.0 / 96.0) * 1000.0) as u16, 395);
    assert_eq!((hlg_oetf(6.0 / 96.0) * 1000.0) as u16, 433);
    assert_eq!((hlg_oetf(7.0 / 96.0) * 1000.0) as u16, 467);

    //
    // INTERSECTION POINT
    //

    assert_eq!((hlg_oetf(1.0 / 12.0) * 1000.0) as u16, 500);

    //
    // UPPER CURVE
    //

    assert_eq!((hlg_oetf(2.0 / 12.0) * 1000.0) as u16, 656);
    assert_eq!((hlg_oetf(3.0 / 12.0) * 1000.0) as u16, 738);
    assert_eq!((hlg_oetf(4.0 / 12.0) * 1000.0) as u16, 794);
    assert_eq!((hlg_oetf(5.0 / 12.0) * 1000.0) as u16, 837);
    assert_eq!((hlg_oetf(6.0 / 12.0) * 1000.0) as u16, 871);
    assert_eq!((hlg_oetf(7.0 / 12.0) * 1000.0) as u16, 900);
    assert_eq!((hlg_oetf(8.0 / 12.0) * 1000.0) as u16, 925);
    assert_eq!((hlg_oetf(9.0 / 12.0) * 1000.0) as u16, 947);
    assert_eq!((hlg_oetf(10.0 / 12.0) * 1000.0) as u16, 966);
    assert_eq!((hlg_oetf(11.0 / 12.0) * 1000.0) as u16, 984);
    assert_eq!((hlg_oetf(12.0 / 12.0) * 1000.0) as u16, 999);
}

#[test]
fn test_pq_hlg() {

    let mut pixel = Pixel { red: 0.58, green: 0.58, blue: 0.58 };

    pixel = Pixel {
        red: pq_eotf(pixel.red),
        green: pq_eotf(pixel.green),
        blue: pq_eotf(pixel.blue),
    };
    pixel = pq_hlg_iootf(pixel);
    pixel = Pixel {
        red: hlg_oetf(pixel.red),
        green: hlg_oetf(pixel.green),
        blue: hlg_oetf(pixel.blue),
    };

    assert_eq!(
        Pixel {
            red: (pixel.red * 100.0).round(),
            green: (pixel.green * 100.0).round(),
            blue: (pixel.blue * 100.0).round(),
        },
        Pixel {
            red: 75.0,
            green: 75.0,
            blue: 75.0,
        }
    );
}
