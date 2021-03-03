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
