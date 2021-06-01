/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

use super::*;

use assert_approx_eq::assert_approx_eq;

const DIFF: f64 = 0.0000000001;

#[test]
fn test_bt2390_overrun_peak_1_000() {

    let pq_ootf = Bt2390ToneMapper::new(0.1);

    for i in 0..1_000 {
        assert!(pq_ootf.map(i as f64 / 10_000.0) < 0.1);
    }

    for i in 1_000..10_000 {
        assert!(pq_ootf.map(i as f64 / 10_000.0) == 0.1);
    }
}

#[test]
fn test_bt2390_overrun_peak_4_000() {

    let pq_ootf = Bt2390ToneMapper::new(0.4);

    for i in 0..4_000 {
        assert!(pq_ootf.map(i as f64 / 10_000.0) < 0.1);
    }

    for i in 4_000..10_000 {
        assert!(pq_ootf.map(i as f64 / 10_000.0) == 0.1);
    }
}

#[test]
fn test_bt2390_overrun_peak_10_000() {

    let pq_ootf = Bt2390ToneMapper::new(1.0);

    for i in 0..9_999 {
        assert!(pq_ootf.map(i as f64 / 10_000.0) < 0.1);
    }
}

#[test]
fn test_bt2390_ootf_peak_1000() {

    let pq_ootf = Bt2390ToneMapper::new(0.1);

    assert_approx_eq!(pq_ootf.map(0.00), 0.0, DIFF);
    assert_approx_eq!(pq_ootf.map(0.01), 0.01, DIFF);
    assert_approx_eq!(pq_ootf.map(0.02), 0.02, DIFF);
    assert_approx_eq!(pq_ootf.map(0.03), 0.03, DIFF);
    assert_approx_eq!(pq_ootf.map(0.04), 0.04, DIFF);
    assert_approx_eq!(pq_ootf.map(0.05), 0.05, DIFF);
    assert_approx_eq!(pq_ootf.map(0.06), 0.06, DIFF);
    assert_approx_eq!(pq_ootf.map(0.07), 0.07, DIFF);
    assert_approx_eq!(pq_ootf.map(0.08), 0.08, DIFF);
    assert_approx_eq!(pq_ootf.map(0.09), 0.09, DIFF);
    assert_approx_eq!(pq_ootf.map(0.10), 0.1, DIFF);
}

#[test]
fn test_bt2390_ootf_peak_2000() {

    let pq_ootf = Bt2390ToneMapper::new(0.2);

    assert_approx_eq!(pq_ootf.map(0.00), 0.0, DIFF);
    assert_approx_eq!(pq_ootf.map(0.02), 0.02, DIFF);
    assert_approx_eq!(pq_ootf.map(0.04), 0.04, DIFF);
    assert_approx_eq!(pq_ootf.map(0.06), 0.06, DIFF);
    assert_approx_eq!(pq_ootf.map(0.08), 0.0788733929849, DIFF);
    assert_approx_eq!(pq_ootf.map(0.10), 0.0902325902087, DIFF);
    assert_approx_eq!(pq_ootf.map(0.12), 0.0959704921902, DIFF);
    assert_approx_eq!(pq_ootf.map(0.14), 0.0986105869303, DIFF);
    assert_approx_eq!(pq_ootf.map(0.16), 0.0996582983608, DIFF);
    assert_approx_eq!(pq_ootf.map(0.18), 0.099964016835, DIFF);
    assert_approx_eq!(pq_ootf.map(0.20), 0.1, DIFF);
}

#[test]
fn test_bt2390_ootf_peak_4000() {

    let pq_ootf = Bt2390ToneMapper::new(0.4);

    assert_approx_eq!(pq_ootf.map(0.0), 0.0, DIFF);
    assert_approx_eq!(pq_ootf.map(0.04), 0.04, DIFF);
    assert_approx_eq!(pq_ootf.map(0.08), 0.0725237577607, DIFF);
    assert_approx_eq!(pq_ootf.map(0.12), 0.087451059744, DIFF);
    assert_approx_eq!(pq_ootf.map(0.16), 0.0942822121197, DIFF);
    assert_approx_eq!(pq_ootf.map(0.20), 0.0974937173873, DIFF);
    assert_approx_eq!(pq_ootf.map(0.24), 0.0989933164328, DIFF);
    assert_approx_eq!(pq_ootf.map(0.28), 0.0996575369944, DIFF);
    assert_approx_eq!(pq_ootf.map(0.32), 0.0999163530966, DIFF);
    assert_approx_eq!(pq_ootf.map(0.36), 0.0999912239036, DIFF);
    assert_approx_eq!(pq_ootf.map(0.4), 0.1, DIFF);
}

#[test]
fn test_bt2390_ootf_peak_8000() {

    let pq_ootf = Bt2390ToneMapper::new(0.8);

    assert_approx_eq!(pq_ootf.map(0.0), 0.0, DIFF);
    assert_approx_eq!(pq_ootf.map(0.08), 0.0659387923192, DIFF);
    assert_approx_eq!(pq_ootf.map(0.16), 0.0868922897571, DIFF);
    assert_approx_eq!(pq_ootf.map(0.24), 0.0943498933538, DIFF);
    assert_approx_eq!(pq_ootf.map(0.32), 0.0974933977663, DIFF);
    assert_approx_eq!(pq_ootf.map(0.40), 0.0989165570687, DIFF);
    assert_approx_eq!(pq_ootf.map(0.48), 0.0995683080591, DIFF);
    assert_approx_eq!(pq_ootf.map(0.56), 0.0998538779943, DIFF);
    assert_approx_eq!(pq_ootf.map(0.64), 0.0999644309626, DIFF);
    assert_approx_eq!(pq_ootf.map(0.72), 0.0999962776243, DIFF);
    assert_approx_eq!(pq_ootf.map(0.8), 0.1, DIFF);
}

#[test]
fn test_bt2390_ootf_peak_10000() {

    let pq_ootf = Bt2390ToneMapper::new(1.0);

    assert_approx_eq!(pq_ootf.map(0.0), 0.0, DIFF);
    assert_approx_eq!(pq_ootf.map(0.1), 0.0713332140595, DIFF);
    assert_approx_eq!(pq_ootf.map(0.2), 0.0892739186715, DIFF);
    assert_approx_eq!(pq_ootf.map(0.3), 0.0954237522859, DIFF);
    assert_approx_eq!(pq_ootf.map(0.4), 0.0979798879174, DIFF);
    assert_approx_eq!(pq_ootf.map(0.5), 0.0991292928898, DIFF);
    assert_approx_eq!(pq_ootf.map(0.6), 0.0996536883985, DIFF);
    assert_approx_eq!(pq_ootf.map(0.7), 0.0998829221806, DIFF);
    assert_approx_eq!(pq_ootf.map(0.8), 0.0999715271039, DIFF);
    assert_approx_eq!(pq_ootf.map(0.9), 0.0999970224486, DIFF);
    assert_approx_eq!(pq_ootf.map(1.0), 0.1, DIFF);
}
