#![allow(unused)]

use std::collections::LinkedList;

fn main() {
    // bounded parametric polymorphism (generics)
    smallest_value();

    /*

    monomorphization (as opposed to dynamic dispatch)

    so how generics are implemented in Rust?
    - Rust uses a technique called monomorphization to generate code for each type that is used with a generic function.
    - This means that the compiler generates a new version of the function for each type that is used with it.
    - This is different from how generics are implemented in some other languages, where the generic function is compiled once and then type-checked at runtime.

    advantages:
    - monomorphization allows Rust to generate very efficient code for generic functions -- i.e., no runtime overhead
    - allows compiler to optimise code (e.g. via inlining) for each specific type that is used with a generic function

    disadvantages:
    - can lead to larger binary sizes, as the compiler generates a new version of the function for each type that is used with it
    - can lead to longer compile times, as the compiler has to generate a new version of the function for each type that is used with it
    - must know what all the types are at compile time -> cannot pre-compile a library and then use it with new types later

    */
}

// suppose I had three functions
fn smallest_value() {
    smallest_i32(1, 2);
    smallest_f32(1.0, 2.0);
    smallest_char('a', 'b');

    // I could write a generic function that works for all of them
    smallest::<i32>(1, 3);
    smallest::<f32>(1.0, 3.0);
    smallest::<char>('a', 'c');
    // we don't need to specify the type if the compiler can infer it
    smallest(1, 3);

    // can make our types derive from the PartialOrd trait
    #[derive(PartialOrd, PartialEq)]
    struct MyType {
        value: i32,
    }
    let a = MyType { value: 1 };
    let b = MyType { value: 2 };
    smallest(a, b);

    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    smallest_general3(&list);
    smallest_general3(list);
}

fn smallest_i32(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}
fn smallest_f32(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}
fn smallest_char(a: char, b: char) -> char {
    if a < b {
        a
    } else {
        b
    }
}

// I could write a generic function that works for all of them
// the syntax T: PartialOrd means that T must implement the PartialOrd trait
fn smallest<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

// can also use a where clause
fn smallest_where<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a < b {
        a
    } else {
        b
    }
}

// can combine with other traits using '+' syntax
fn smallest_display<T>(a: T, b: T) -> T
where
    T: PartialOrd + std::fmt::Display,
{
    if a < b {
        a
    } else {
        b
    }
}

// can make more useful by making it work for any number of elements
fn smallest_general<T>(xs: Vec<T>) -> T
where
    T: PartialOrd + Default, // use a type that has a default value?
{
    todo!()
}

fn smallest_general2<T>(xs: Vec<T>) -> Option<T>
// return an Option to handle empty lists
where
    T: PartialOrd + Default,
{
    let mut iter = xs.into_iter();
    let mut smallest = iter.next()?;

    for x in iter {
        if x < smallest {
            smallest = x;
        }
    }

    Some(smallest)
}

// what's not so great about smallest_general2?
// its parameter is a Vec -> but only because we want a list of things. We don't actually need a Vec specifically -- it is too concrete.
fn smallest_general3<T, I>(xs: I) -> Option<T>
where
    T: PartialOrd,
    I: IntoIterator<Item = T>, // associated type
{
    let mut iter = xs.into_iter();
    let mut smallest = iter.next()?;

    for x in iter {
        if x < smallest {
            smallest = x;
        }
    }

    Some(smallest)
}

// another way to write the above
fn smallest_general4<T>(xs: impl IntoIterator<Item = T>) -> Option<T>
where
    T: PartialOrd,
{
    let mut iter = xs.into_iter();
    let mut smallest = iter.next()?;

    for x in iter {
        if x < smallest {
            smallest = x;
        }
    }

    Some(smallest)
}

// we can use generics with impl statements too
struct MyStruct<T> {
    value: T,
}

impl<T> MyStruct<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> MyStruct<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            value: T::default(),
        }
    }
}
