// Copyright Jeron A. Lau 2017-2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

#![allow(unused)]

// Copied from https://github.com/DougLau/footile
/// Scale a u8 value by another (for alpha blending)
#[inline(always)]
pub(crate) fn scale_u8(a: u8, b: u8) -> u8 {
    // cheap alternative to divide by 255
    let c = a as u32 * b as u32;
    let c = ((c + 1) + (c >> 8)) >> 8;
    c as u8
}

// Copied from https://github.com/DougLau/footile
/// Unscale a u8
#[inline(always)]
pub(crate) fn unscale_u8(a: u8, b: u8) -> u8 {
    if b > 0 {
        let aa = (a as u32) << 8;
        let bb = b as u32;
        (aa / bb).min(255) as u8
    } else {
        0
    }
}
