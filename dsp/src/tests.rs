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

use super::{
    *,
    tm::ToneMapMethod,
};
use assert_approx_eq::assert_approx_eq;

const HDR_DIFF: f64 = 0.000001;
const HLG_BLACK: f64 = 0.0;
const HLG_REF_WHITE: f64 = 0.7498773;
const HLG_MAX_WHITE: f64 = 1.0;
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
const SDR_DIFF: f64 = 0.00001;
const SDR_BLACK: f64 = 0.0;
const SDR_GREY_18: f64 = 0.3870226424119708;
const SDR_GREY_83: f64 = 0.8294599123812822;
const SDR_GREY_90: f64 = 0.8646750115119883;
const SDR_REF_WHITE: f64 = 0.9112149320796772;

#[test]
fn test_pq_hlg_map_rw_100_peak_492() {

    let mut frame = vec![
        RgbPixel { red: PQ_BLACK, green: PQ_BLACK, blue: PQ_BLACK },
        RgbPixel { red: PQ_100_NITS, green: PQ_100_NITS, blue: PQ_100_NITS },
        RgbPixel { red: PQ_492_NITS, green: PQ_492_NITS, blue: PQ_492_NITS },
        RgbPixel { red: PQ_4000_NITS, green: PQ_4000_NITS, blue: PQ_4000_NITS },
        RgbPixel { red: PQ_10000_NITS, green: PQ_10000_NITS, blue: PQ_10000_NITS },
    ];
    let pq_hlg_mapper =
        PqHlgMapper::new_by_ref_white(100.0, 0.49261083743842365, ToneMapMethod::MaxRgb, false);

    for pixel in frame.iter_mut() {
        *pixel = pq_hlg_mapper.map(*pixel);
    }

    assert_approx_eq!(frame[0].red, HLG_BLACK, HDR_DIFF);
    assert_approx_eq!(frame[0].green, HLG_BLACK, HDR_DIFF);
    assert_approx_eq!(frame[0].blue, HLG_BLACK, HDR_DIFF);

    assert_approx_eq!(frame[1].red, HLG_REF_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[1].green, HLG_REF_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[1].blue, HLG_REF_WHITE, HDR_DIFF);

    assert_approx_eq!(frame[2].red, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[2].green, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[2].blue, HLG_MAX_WHITE, HDR_DIFF);

    assert_approx_eq!(frame[3].red, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[3].green, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[3].blue, HLG_MAX_WHITE, HDR_DIFF);

    assert_approx_eq!(frame[4].red, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[4].green, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[4].blue, HLG_MAX_WHITE, HDR_DIFF);
}

#[test]
fn test_pq_hlg_map_rw_100_peak_1_970() {

    let mut frame = vec![
        RgbPixel { red: PQ_BLACK, green: PQ_BLACK, blue: PQ_BLACK },
        RgbPixel { red: PQ_100_NITS, green: PQ_100_NITS, blue: PQ_100_NITS },
        RgbPixel { red: PQ_1970_NITS, green: PQ_1970_NITS, blue: PQ_1970_NITS },
        RgbPixel { red: PQ_4000_NITS, green: PQ_4000_NITS, blue: PQ_4000_NITS },
        RgbPixel { red: PQ_10000_NITS, green: PQ_10000_NITS, blue: PQ_10000_NITS },
    ];
    let pq_hlg_mapper =
        PqHlgMapper::new_by_ref_white(100.0, 1970.4433497536945, ToneMapMethod::MaxRgb, false);

    for pixel in frame.iter_mut() {
        *pixel = pq_hlg_mapper.map(*pixel);
    }

    assert_approx_eq!(frame[0].red, HLG_BLACK, HDR_DIFF);
    assert_approx_eq!(frame[0].green, HLG_BLACK, HDR_DIFF);
    assert_approx_eq!(frame[0].blue, HLG_BLACK, HDR_DIFF);

    assert_approx_eq!(frame[1].red, HLG_REF_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[1].green, HLG_REF_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[1].blue, HLG_REF_WHITE, HDR_DIFF);

    assert_approx_eq!(frame[2].red, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[2].green, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[2].blue, HLG_MAX_WHITE, HDR_DIFF);

    assert_approx_eq!(frame[3].red, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[3].green, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[3].blue, HLG_MAX_WHITE, HDR_DIFF);

    assert_approx_eq!(frame[4].red, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[4].green, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[4].blue, HLG_MAX_WHITE, HDR_DIFF);
}

#[test]
fn test_pq_hlg_map_rw_100_peak_4_926() {

    let mut frame = vec![
        RgbPixel { red: PQ_BLACK, green: PQ_BLACK, blue: PQ_BLACK },
        RgbPixel { red: PQ_100_NITS, green: PQ_100_NITS, blue: PQ_100_NITS },
        RgbPixel { red: PQ_4926_NITS, green: PQ_4926_NITS, blue: PQ_4926_NITS },
        RgbPixel { red: PQ_10000_NITS, green: PQ_10000_NITS, blue: PQ_10000_NITS },
    ];
    let pq_hlg_mapper = PqHlgMapper::new_by_ref_white(100.0, 4_926.0, ToneMapMethod::MaxRgb, false);

    for pixel in frame.iter_mut() {
        *pixel = pq_hlg_mapper.map(*pixel);
    }

    assert_approx_eq!(frame[0].red, HLG_BLACK, HDR_DIFF);
    assert_approx_eq!(frame[0].green, HLG_BLACK, HDR_DIFF);
    assert_approx_eq!(frame[0].blue, HLG_BLACK, HDR_DIFF);

    assert_approx_eq!(frame[1].red, HLG_REF_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[1].green, HLG_REF_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[1].blue, HLG_REF_WHITE, HDR_DIFF);

    assert_approx_eq!(frame[2].red, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[2].green, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[2].blue, HLG_MAX_WHITE, HDR_DIFF);

    assert_approx_eq!(frame[3].red, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[3].green, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[3].blue, HLG_MAX_WHITE, HDR_DIFF);
}

#[test]
fn test_pq_hlg_map_rw_203_peak_500() {

    let mut frame = vec![
        RgbPixel { red: PQ_BLACK, green: PQ_BLACK, blue: PQ_BLACK },
        RgbPixel { red: PQ_REF_WHITE, green: PQ_REF_WHITE, blue: PQ_REF_WHITE },
        RgbPixel { red: PQ_1000_NITS, green: PQ_1000_NITS, blue: PQ_1000_NITS },
        RgbPixel { red: PQ_4000_NITS, green: PQ_4000_NITS, blue: PQ_4000_NITS },
        RgbPixel { red: PQ_10000_NITS, green: PQ_10000_NITS, blue: PQ_10000_NITS },
    ];
    let pq_hlg_mapper = PqHlgMapper::new_by_ref_white(203.0, 500.0, ToneMapMethod::MaxRgb, false);

    for pixel in frame.iter_mut() {
        *pixel = pq_hlg_mapper.map(*pixel);
    }

    assert_approx_eq!(frame[0].red, HLG_BLACK, HDR_DIFF);
    assert_approx_eq!(frame[0].green, HLG_BLACK, HDR_DIFF);
    assert_approx_eq!(frame[0].blue, HLG_BLACK, HDR_DIFF);

    assert_approx_eq!(frame[1].red, HLG_REF_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[1].green, HLG_REF_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[1].blue, HLG_REF_WHITE, HDR_DIFF);

    assert_approx_eq!(frame[2].red, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[2].green, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[2].blue, HLG_MAX_WHITE, HDR_DIFF);

    assert_approx_eq!(frame[3].red, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[3].green, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[3].blue, HLG_MAX_WHITE, HDR_DIFF);

    assert_approx_eq!(frame[4].red, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[4].green, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[4].blue, HLG_MAX_WHITE, HDR_DIFF);
}

#[test]
fn test_pq_hlg_map_rw_203_peak_1_000() {

    let mut frame = vec![
        RgbPixel { red: PQ_BLACK, green: PQ_BLACK, blue: PQ_BLACK },
        RgbPixel { red: PQ_REF_WHITE, green: PQ_REF_WHITE, blue: PQ_REF_WHITE },
        RgbPixel { red: PQ_1000_NITS, green: PQ_1000_NITS, blue: PQ_1000_NITS },
        RgbPixel { red: PQ_4000_NITS, green: PQ_4000_NITS, blue: PQ_4000_NITS },
        RgbPixel { red: PQ_10000_NITS, green: PQ_10000_NITS, blue: PQ_10000_NITS },
    ];
    let pq_hlg_mapper = PqHlgMapper::new_by_ref_white(203.0, 1_000.0, ToneMapMethod::MaxRgb, false);

    for pixel in frame.iter_mut() {
        *pixel = pq_hlg_mapper.map(*pixel);
    }

    assert_approx_eq!(frame[0].red, HLG_BLACK, HDR_DIFF);
    assert_approx_eq!(frame[0].green, HLG_BLACK, HDR_DIFF);
    assert_approx_eq!(frame[0].blue, HLG_BLACK, HDR_DIFF);

    assert_approx_eq!(frame[1].red, HLG_REF_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[1].green, HLG_REF_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[1].blue, HLG_REF_WHITE, HDR_DIFF);

    assert_approx_eq!(frame[2].red, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[2].green, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[2].blue, HLG_MAX_WHITE, HDR_DIFF);

    assert_approx_eq!(frame[3].red, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[3].green, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[3].blue, HLG_MAX_WHITE, HDR_DIFF);

    assert_approx_eq!(frame[4].red, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[4].green, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[4].blue, HLG_MAX_WHITE, HDR_DIFF);
}

#[test]
fn test_pq_hlg_map_rw_203_peak_4_000() {

    let mut frame = vec![
        RgbPixel { red: PQ_BLACK, green: PQ_BLACK, blue: PQ_BLACK },
        RgbPixel { red: PQ_REF_WHITE, green: PQ_REF_WHITE, blue: PQ_REF_WHITE },
        RgbPixel { red: PQ_4000_NITS, green: PQ_4000_NITS, blue: PQ_4000_NITS },
        RgbPixel { red: PQ_10000_NITS, green: PQ_10000_NITS, blue: PQ_10000_NITS },
    ];
    let pq_hlg_mapper = PqHlgMapper::new_by_ref_white(203.0, 4_000.0, ToneMapMethod::MaxRgb, false);

    for pixel in frame.iter_mut() {
        *pixel = pq_hlg_mapper.map(*pixel);
    }

    assert_approx_eq!(frame[0].red, HLG_BLACK, HDR_DIFF);
    assert_approx_eq!(frame[0].green, HLG_BLACK, HDR_DIFF);
    assert_approx_eq!(frame[0].blue, HLG_BLACK, HDR_DIFF);

    assert_approx_eq!(frame[1].red, HLG_REF_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[1].green, HLG_REF_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[1].blue, HLG_REF_WHITE, HDR_DIFF);

    assert_approx_eq!(frame[2].red, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[2].green, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[2].blue, HLG_MAX_WHITE, HDR_DIFF);

    assert_approx_eq!(frame[3].red, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[3].green, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[3].blue, HLG_MAX_WHITE, HDR_DIFF);
}

#[test]
fn test_pq_hlg_map_rw_203_peak_10_000() {

    let mut frame = vec![
        RgbPixel { red: PQ_BLACK, green: PQ_BLACK, blue: PQ_BLACK },
        RgbPixel { red: PQ_REF_WHITE, green: PQ_REF_WHITE, blue: PQ_REF_WHITE },
        RgbPixel { red: PQ_10000_NITS, green: PQ_10000_NITS, blue: PQ_10000_NITS },
    ];
    let pq_hlg_mapper = PqHlgMapper::new_by_ref_white(203.0, 10_000.0, ToneMapMethod::MaxRgb, false);

    for pixel in frame.iter_mut() {
        *pixel = pq_hlg_mapper.map(*pixel);
    }

    assert_approx_eq!(frame[0].red, HLG_BLACK, HDR_DIFF);
    assert_approx_eq!(frame[0].green, HLG_BLACK, HDR_DIFF);
    assert_approx_eq!(frame[0].blue, HLG_BLACK, HDR_DIFF);

    assert_approx_eq!(frame[1].red, HLG_REF_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[1].green, HLG_REF_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[1].blue, HLG_REF_WHITE, HDR_DIFF);

    assert_approx_eq!(frame[2].red, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[2].green, HLG_MAX_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[2].blue, HLG_MAX_WHITE, HDR_DIFF);
}

#[test]
fn test_preview_map() {

    let mut frame = vec![
        RgbPixel { red: PQ_BLACK, green: PQ_BLACK, blue: PQ_BLACK },
        RgbPixel { red: PQ_REF_WHITE, green: PQ_REF_WHITE, blue: PQ_REF_WHITE },
    ];
    let pq_hlg_mapper = PqSdrMapper::new_by_factor(1.0, 10_000.0, ToneMapMethod::MaxRgb);

    for pixel in frame.iter_mut() {
        *pixel = pq_hlg_mapper.map(*pixel);
    }

    assert_approx_eq!(frame[0].red, SDR_BLACK, SDR_DIFF);
    assert_approx_eq!(frame[0].green, SDR_BLACK, SDR_DIFF);
    assert_approx_eq!(frame[0].blue, SDR_BLACK, SDR_DIFF);

    assert_approx_eq!(frame[1].red, SDR_REF_WHITE, SDR_DIFF);
    assert_approx_eq!(frame[1].green, SDR_REF_WHITE, SDR_DIFF);
    assert_approx_eq!(frame[1].blue, SDR_REF_WHITE, SDR_DIFF);
}

#[test]
fn test_hlg_pq_map_1_000() {

    let mut frame = vec![
        RgbPixel { red: HLG_BLACK, green: HLG_BLACK, blue: HLG_BLACK },
        RgbPixel { red: HLG_REF_WHITE, green: HLG_REF_WHITE, blue: HLG_REF_WHITE },
        RgbPixel { red: HLG_MAX_WHITE, green: HLG_MAX_WHITE, blue: HLG_MAX_WHITE },
    ];
    let hlg_pq_mapper = HlgPqMapper::new(1_000.0);

    for pixel in frame.iter_mut() {
        *pixel = hlg_pq_mapper.map(*pixel);
    }

    assert_approx_eq!(frame[0].red, PQ_BLACK, HDR_DIFF);
    assert_approx_eq!(frame[0].green, PQ_BLACK, HDR_DIFF);
    assert_approx_eq!(frame[0].blue, PQ_BLACK, HDR_DIFF);

    assert_approx_eq!(frame[1].red, PQ_REF_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[1].green, PQ_REF_WHITE, HDR_DIFF);
    assert_approx_eq!(frame[1].blue, PQ_REF_WHITE, HDR_DIFF);

    assert_approx_eq!(frame[2].red, PQ_1000_NITS, HDR_DIFF);
    assert_approx_eq!(frame[2].green, PQ_1000_NITS, HDR_DIFF);
    assert_approx_eq!(frame[2].blue, PQ_1000_NITS, HDR_DIFF);
}
