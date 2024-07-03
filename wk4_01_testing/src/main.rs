#![allow(unused)]

// cargo test or cargo t        to run all tests
// cargo test --lib             to run only unit tests
// cargo test --test my_test    to run only integration test
// cargo test --doc             to run doc tests
// cargo doc                    to generate documentation

use wk4_01_testing::my_add;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_1() {
    assert_eq!(1, 1);
}

#[test] // test attribute is used to define a test function
fn test_2() {
    assert_eq!(my_add(1, 2), 3);
    assert_eq!(my_add(1, 3), 4);
}

// common pattern is to put tests in a separate module
#[cfg(test)] // config test = only compile this module when running tests
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(my_add(3, 5), 8);
    }
}

// look in std::assert for more assertions
