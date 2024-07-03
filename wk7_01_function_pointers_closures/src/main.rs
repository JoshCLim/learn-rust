#![allow(unused)]

use std::time::{Duration, Instant};

fn main() {
    // basic function pointers
    infinite_loop();

    // example: map function
    use_my_map();

    // limitation of function pointers -> no environment
    function_pointers_limitations();

    // my_map with closure
    use_my_better_map();

    // function traits: Fn, FnOnce, FnMut
    use_my_even_better_map();

    // example: using FnOnce
    use_time_closure();
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
fn infinite_loop() {
    // specify a function type using
    // fn(T, U, V, ...) -> R
    // e.g. `fn()` for a function that takes no arguments and returns nothing
    // e.g. `fn() -> i32` for a function that takes no arguments and returns an i32
    // e.g. `fn(char)` for a function that takes a char argument and returns nothing
    // e.g. `fn(i32, i32) -> i32` for a function that takes two i32 arguments and returns an i32

    let my_add: fn(i32, i32) -> i32 = add;
    let result = my_add(1, 2);

    println!("This will run forever! {}", result);

    let my_func: fn() = infinite_loop;
    my_func();
}

// what my_map will return
struct MyMap<I, OldItem, NewItem> {
    iter: I,
    func: fn(OldItem) -> NewItem,
}
impl<I, OldItem, NewItem> Iterator for MyMap<I, OldItem, NewItem>
where
    I: Iterator<Item = OldItem>,
{
    type Item = NewItem;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.iter.next()?;
        Some((self.func)(curr))
    }
}
fn my_map<I, OldItem, NewItem>(
    iter: I,
    func: fn(OldItem) -> NewItem,
) -> impl Iterator<Item = NewItem>
where
    I: Iterator<Item = OldItem>,
{
    MyMap { iter, func }
}

fn use_my_map() {
    let numbers = vec![1, 2, 3, 4, 5];
    let squared = my_map(numbers.into_iter(), |x| x * x)
        .filter(|x| *x < 5)
        .collect::<Vec<_>>();
    println!("{:?}", squared);
}

// limitation of function pointers -> no environment
fn function_pointers_limitations() {
    let factor = 2.5;
    let vec = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    // error: since the closure captures the environment (i.e., uses the `factor` variable), it cannot be converted to a function pointer
    // my_map(vec.into_iter(), |x| x * factor).collect::<Vec<_>>();

    // we need CLOSURES
    // basically a function pointer + environment
    //
    // a closure has 0 or more captures
    // a closure with 0 captures can be treated as a function pointer
    //
    // syntax for declaring a closure:
    //  |x,y,z...| { body }
    let closure = |x: i32| x * 2;
    //
    // type of a closure cannot be named whoaa
    // instead, closures implement the `Fn`, `FnMut`, and `FnOnce` traits
    // which allow us to use them
    //
    // how are closures handled by the compiler?
    // - the compiler generates a struct for the closure
    // - the struct has a field for each captured variable
    // - the struct implements the `Fn`, `FnMut`, or `FnOnce` trait
    // - the struct implements the `call` method
    // and then anywhere in our function body we use a captured variable, the compiler will replace it with the field in the closure struct
}

struct MyBetterMap<I, F> {
    iter: I,
    func: F,
}
impl<I, F, OldItem, NewItem> Iterator for MyBetterMap<I, F>
where
    I: Iterator<Item = OldItem>,
    F: Fn(OldItem) -> NewItem,
{
    type Item = NewItem;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.iter.next()?;
        Some((self.func)(curr))
    }
}
fn my_better_map<I, F, OldItem, NewItem>(iter: I, func: F) -> impl Iterator<Item = NewItem>
where
    I: Iterator<Item = OldItem>,
    F: Fn(OldItem) -> NewItem,
{
    MyBetterMap { iter, func }
}
fn use_my_better_map() {
    let numbers = vec![1, 2, 3, 4, 5];
    let factor = 2;
    let squared = my_better_map(numbers.into_iter(), |x| x * x * factor)
        .filter(|x| *x < 5)
        .collect::<Vec<_>>();
    println!("{:?}", squared);

    // here is a case where the better_map doesn't work
    let mut nth = 0;
    let names = vec!["Alice", "Bob", "Carol"];
    let greetings = names.clone().into_iter().map(|name| {
        let res = format!("#{}: Hello, {}!", nth, name);
        nth += 1;

        res
    });

    let mut my_nth = 0;
    let my_greetings = my_better_map(names.clone().into_iter(), |name| {
        let res = format!("#{}: Hello, {}!", nth, name);
        // nth += 1; // error: cannot borrow `nth` as mutable more than once at a time

        res
    })
    .collect::<Vec<_>>();

    // why?
    // better_map uses Fn, which is a trait that denotes a function that can be called indefinitely simultaneously
    // so, we can't take a mutable borrow
    // instead, we can use FnMut, which is a trait that denotes a function that can be called indefinitely, but never simultaneously -> allows mutable borrows
    // or FnOnce, which is a trait that denotes a function that can be called only once -> allows moves
    //
    // so in order to call an FnMut, we need access to &mut self
    // in order to call an FnOnce, we need access to self
    // in order to call an Fn, we need access to &self
    //
    // the subtypes for these traits:
    // FnOnce > FnMut > Fn > fn
    //      i.e., all fn's can be treated as Fn's, Fn's can be treated as FnMut's, etc.
    // when you design an API, try to use the least restrictive trait possible (but keep forward-compatibility in mind)
    //      e.g. you would rather use FnMut than Fn
    //
    // if the closure captures variables in all three ways, it will be treated as the most restrictive trait
}

struct MyEvenBetterMap<I, F> {
    iter: I,
    func: F,
}
impl<I, F, OldItem, NewItem> Iterator for MyEvenBetterMap<I, F>
where
    I: Iterator<Item = OldItem>,
    F: FnMut(OldItem) -> NewItem,
{
    type Item = NewItem;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.iter.next()?;
        Some((self.func)(curr))
    }
}
fn my_even_better_map<I, F, OldItem, NewItem>(iter: I, func: F) -> impl Iterator<Item = NewItem>
where
    I: Iterator<Item = OldItem>,
    F: FnMut(OldItem) -> NewItem,
{
    MyEvenBetterMap { iter, func }
}
fn use_my_even_better_map() {
    let numbers = vec![1, 2, 3, 4, 5];
    let factor = 2;
    let squared = my_even_better_map(numbers.into_iter(), |x| x * x * factor)
        .filter(|x| *x < 5)
        .collect::<Vec<_>>();
    println!("{:?}", squared);

    // here is a case where the better_map doesn't work
    let mut nth = 0;
    let names = vec!["Alice", "Bob", "Carol"];
    let greetings = names.clone().into_iter().map(|name| {
        let res = format!("#{}: Hello, {}!", nth, name);
        nth += 1;

        res
    });

    let mut my_nth = 0;
    let my_greetings = my_even_better_map(names.clone().into_iter(), |name| {
        let res = format!("#{}: Hello, {}!", nth, name);
        nth += 1; // error: cannot borrow `nth` as mutable more than once at a time

        res
    })
    .collect::<Vec<_>>();

    // why?
    // better_map uses Fn, which is a trait that denotes a function that can be called indefinitely simultaneously
    // so, we can't take a mutable borrow
    // instead, we can use FnMut, which is a trait that denotes a function that can be called indefinitely, but never simultaneously
}

// example: using closures to time a function
fn time_closure<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let before = Instant::now();
    let res = f();
    (res, Instant::now().duration_since(before))
}

fn use_time_closure() {
    let (_, duration) = time_closure(|| {
        let mut sum = 0;
        for i in 0..1_000_000 {
            sum += i;
        }
    });
    println!("Time taken: {:?}", duration);
}
