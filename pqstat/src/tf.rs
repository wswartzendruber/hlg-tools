/*
 * Copyright © 2021 William Swartzendruber
 *
 * Licensed under the Open Software License (OSL 3.0).
 */

#[cfg(test)]
mod tests;

pub fn pq_eotf(e: f64) -> f64 {

    //
    // ITU-R BT.2100-2
    // Table 4
    //

    (
        (e.powf(0.012683313515655966) - 0.8359375).max(0.0)
        /
        (18.8515625 - 18.6875 * e.powf(0.012683313515655966))
    )
    .powf(6.277394636015326)
}
