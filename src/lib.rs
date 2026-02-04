#![allow(clippy::too_many_arguments)]

use std::cmp::PartialOrd;
use std::ops::{Add, Sub};

mod bitblock_transfer;

pub mod bgra_management;
pub mod legacy;
pub mod pixels;
pub mod screen;

pub use screen::*;

#[cfg(feature = "pixels_string")]
pub mod pixels_string;

/// Keeps adding/subtracting units of `add` to `val` until it reaches `limit` or `add` has no more units.
pub fn apply_limited_delta<T>(val: T, add: T, limit: T) -> T
where
    T: Add<Output = T> + Sub<Output = T> + PartialOrd + Copy + std::cmp::Ord,
    T: Default,
{
    let zero = T::default();
    let new_val = val + add;

    if add > zero {
        // If adding, we can't exceed the limit, but we also
        // shouldn't decrease if we're already above it.
        new_val.min(val.max(limit))
    } else {
        // If subtracting, we can't go below the limit,
        // but we shouldn't increase if we're already below it.
        new_val.max(val.min(limit))
    }
}

/// ternary (c# alias for °statement° ? °true case° : °false case° )
/// if $test is true return $true_expr, else return $false_expr
// Example of how to use macro:
// ternary!(foo == bar => println!("it is true, they are equal"); println!("it is false, they are not equal"));
#[macro_export]
macro_rules! ternary {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test { $true_expr } else { $false_expr }
    };
}
