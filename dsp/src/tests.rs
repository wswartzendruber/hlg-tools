/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

use super::*;

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

    assert_eq!(frame[0], Pixel {
        red: 0.0,
        green: 0.0,
        blue: 0.0
    });
    assert_eq!(frame[1], Pixel {
        red: 0.7487980450264469,
        green: 0.7487980450264469,
        blue: 0.7487980450264469,
    });
    assert_eq!(frame[2], Pixel {
        red: 0.9974408886311783,
        green: 0.9974408886311783,
        blue: 0.9974408886311783,
    });
    assert_eq!(frame[3], Pixel {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    });
}

#[test]
fn test_map_peak_4_000() {

    let mut frame = vec![
        Pixel { red: 0.00, green: 0.00, blue: 0.00 },
        Pixel { red: 0.58, green: 0.58, blue: 0.58 },
        Pixel { red: 0.75, green: 0.75, blue: 0.75 },
        Pixel { red: 0.90, green: 0.90, blue: 0.90 },
        Pixel { red: 1.00, green: 1.00, blue: 1.00 },
    ];
    let pq_hlg_mapper = PqHlgMapper::new(203.0, 4_000.0);

    for pixel in frame.iter_mut() {
        *pixel = pq_hlg_mapper.map(*pixel);
    }

    assert_eq!(frame[0], Pixel {
        red: 0.0,
        green: 0.0,
        blue: 0.0
    });
    assert_eq!(frame[1], Pixel {
        red: 0.7487980450264478,
        green: 0.7487980450264478,
        blue: 0.7487980450264478,
    });
    assert_eq!(frame[2], Pixel {
        red: 0.9674729779146021,
        green: 0.9674729779146021,
        blue: 0.9674729779146021,
    });
    assert_eq!(frame[3], Pixel {
        red: 0.999999839666903,
        green: 0.999999839666903,
        blue: 0.999999839666903,
    });
    assert_eq!(frame[4], Pixel {
        red: 0.9999999950661305,
        green: 0.9999999950661305,
        blue: 0.9999999950661305,
    });
}
