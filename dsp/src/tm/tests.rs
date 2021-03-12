/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

use super::*;

#[test]
fn test_pq_ootf_peak_1_000() {

    let pq_ootf = ToneMapper::new(0.1);

    for i in 0..1_000 {
        assert!(pq_ootf.map(i as f64 / 10_000.0) < 0.1);
    }

    for i in 1_000..10_000 {
        assert!(pq_ootf.map(i as f64 / 10_000.0) == 0.1);
    }
}

#[test]
fn test_pq_ootf_peak_4_000() {

    let pq_ootf = ToneMapper::new(0.4);

    for i in 0..4_000 {
        assert!(pq_ootf.map(i as f64 / 10_000.0) < 0.1);
    }

    for i in 4_000..10_000 {
        assert!(pq_ootf.map(i as f64 / 10_000.0) == 0.1);
    }
}

#[test]
fn test_pq_ootf_peak_10_000() {

    let pq_ootf = ToneMapper::new(1.0);

    for i in 0..9_999 {
        assert!(pq_ootf.map(i as f64 / 10_000.0) < 0.1);
    }
}
