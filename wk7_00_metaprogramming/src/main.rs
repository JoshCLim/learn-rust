#![allow(unused)]

pub mod proc_macro;

/// # metaprogramming
///
/// what is metaprogramming?
/// - metaprogramming is a programming technique in which computer programs have the ability to treat other programs as their data
///
/// ## macro
/// one example of metaprogramming is macros
///
/// macro = a function from code to code
///
/// why use macros?
/// - to reduce code duplication (convenience)
/// - to "simulate" polymorphism (generally avoided in Rust but sometimes useful esp in languages that don't have good support for polymorphism e.g. C)
/// - syntax extension -- creating syntactic sugar
///
/// ## declarative macros
///
/// declarative macros are a way to generate code at compile time -- basically text substitution
///
/// advantages:
/// - easy to write
/// - easy to read
/// - simple semantics
///
/// disadvantages:
/// - limited in capability
///
/// ## procedural macros
///
/// procedural macros are a way to generate code at compile time by writing a function-like macro
///
/// basically a fn(TokenStream) -> TokenStream
///
/// advantages:
/// - extremely powerful
///
/// disadvantages:
/// - harder to write
/// - harder to read
fn main() {
    use_vec_macro();

    use_up_to();

    use_sort();
}

// the macro_rules! keyword declares the new macro, called `vec`
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    // if you match on `item` it looks for the literal string `item` instead of treating it as a parameter
    // instead we match on `$item:expr` which means "match an expression" and store in variable `$item`
    // `expr` is a Rust macro metavariable that matches any expression
    // other examples of metavariables include `block`, `stmt`, `pat`, `path`, `ty`, `ident`, `lifetime`, `literal`, `meta`, `tt`
    ($item:expr) => {{ // note: the one below this makes this one redundant
        let mut v = Vec::new();
        v.push($item);
        v
    }};
    // the `+` means "one or more", the comma tells us that the repeated string is comma-separated
    // the $(,)? allows for an optional trailing comma
    ($($item:expr),+ $(,)?) => {{
        let mut v = Vec::new();
        $(
            v.push($item);
        )+
        v
    }};
    ($item:expr; $count:expr) => {{
        let mut v = Vec::new();
        for _ in 0..$count {
            v.push($item);
        }
        v
    }};
}

// using the my_vec! macro
fn use_vec_macro() {
    // use `cargo expand` to see the expanded code
    let v0: Vec<i32> = my_vec![];
    let v1: Vec<i32> = my_vec![1];
    let v2: Vec<i32> = my_vec![1, 2, 3];
    let v3: Vec<i32> = my_vec![1, 2, 3,];
    let v4: Vec<i32> = my_vec![0; 10];

    println!("{:?}", v0);
    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("{:?}", v3);
    println!("{:?}", v4);
}

// max macro
macro_rules! find_max {
    ($x:expr, $y:expr) => {{
        // if $x > $y { $x } else { $y } // suffers from function call issue in macros.c
        let a = $x;
        let b = $y;
        if a > b { a } else { b }
    }};
    ($x:expr, $($y:expr),+) => (
        std::cmp::max($x, find_max!($($y),+))
    );
}

// use the find_max! macro
fn use_max_macro() {
    let a = find_max!(1, 2);
    println!("{}", a);
}

// up to macro
macro_rules! up_to {
    ($var:ident, $n:expr, $body: block) => {
        for $var in 0..$n {
            $body
        }
    };
}

fn use_up_to() {
    up_to!(foo, 10, { println!("{}", foo) });
}

// sort macro
use paste::paste;
macro_rules! declare_sort {
    ($prefix:ident, $type:ty) => {
        paste! {
            use std::cmp::Ordering;

            fn [<_declare_sort_ $prefix _compare>](x: &$type, y: &$type) -> Ordering {
                if x > y {
                    Ordering::Greater
                } else if x < y {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }

            fn [<$prefix _sort>](slice: &mut [$type]) {
                slice.sort();
            }
        }
    };
}

declare_sort!(int, i32);

fn use_sort() {
    let mut v = vec![3, 2, 1];
    int_sort(&mut v);
    println!("{:?}", v);
}

// cfor -- using a macro for syntactic sugar
macro_rules! cfor { // BROKEN: since it doesn't work for break and continue
    (for ($($init:stmt),+; $cond:expr; $step:stmt) $body:block) => {{
        $(
            $init
        )+

        while $cond {
            $body
            $step
        }
    }};
}

fn use_cfor() {
    cfor! {
        for (let mut i = 0; i < 10; i += 1) {
            println!("{i}");
        }
    }
}

// use MakroKata to practice writing macros in Rust
