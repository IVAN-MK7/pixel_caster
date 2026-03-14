#![allow(clippy::too_many_arguments)]

mod bitblock_transfer;

pub mod bgra_management;
pub mod legacy;
pub mod pixels;
pub mod screen;
pub mod zero_copy_screen;

pub use screen::*;

#[cfg(feature = "pixels_string")]
pub mod pixels_string;

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
