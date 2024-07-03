//! # wk4_01_testing
//!
//! '//!' means don't doucment the thing that comes next, document the thing i'm in

#![allow(unused)]
#![warn(missing_docs)] // warn if there are missing docs

// #![deny(missing_docs)] // stop compilation if there are missing docs

use core::panic;

// triple slash comments are used for doc tests, and use markdown syntax
// we can put tests within the comments in ``` blocks```, which we can then run as tests

/// # Foo
struct Foo {
    x: i32,
}

/// # my_add
///
/// This function adds two numbers
///
/// # Warning
///
/// This function is used to add two numbers but panics if a or b are less than zero
///
/// # Examples
/// ```
/// # println!("This line of code is included in the test but not in the docs");
/// let result = wk4_01_testing::my_add(1, 2);
/// assert_eq!(result, 3);
/// ```
///
/// # Cool doc features
/// I can link the [`Foo`] struct with square brackets and backticks
pub fn my_add(a: i32, b: i32) -> i32 {
    if a < 0 || b < 0 {
        panic!("a and b must be greater than zero");
    }
    a + b
}
