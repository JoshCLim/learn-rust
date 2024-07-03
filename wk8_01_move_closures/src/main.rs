#![allow(unused)]

fn closure_capturing_borrow() {
    // [EXAMPLE 1: moving a value into a closure]
    let mut x = String::from("x");

    std::thread::spawn(|| {
        let x = x;
        println!("x: {}", x);
    })
    .join()
    .unwrap();

    // println!("my_number: {}", my_number); // error: since we moved my_number into the closure, we can't use it here

    // [EXAMPLE 2 - using a borrow to avoid moving the value into the closure]
    // alternative: use a borrow?
    let mut y = String::from("y");

    // std::thread::spawn(|| { // error: cannot move out of `y` because it is borrowed
    //     let y = &y;
    //     println!("y: {}", y);
    // })
    // .join()
    // .unwrap();

    println!("y: {}", y);

    // [EXAMPLE 3 - using 'move' keyword to force the closure to take ownership of the values it uses]
    // use 'move' keyword
    let mut z = String::from("z");

    // move keyword: forces the closure to take ownership of the values it uses
    std::thread::spawn(move || {
        let z = &z;
        println!("z: {}", z);
    })
    .join()
    .unwrap();

    // println!("z: {}", z); // error: since we moved z into the closure, we can't use it here

    // but importantly, move closures can still implement the 'Fn' and 'FnMut' traits

    // when spawning threads, we can use the 'move' keyword to force the closure to take ownership of the values it uses
}

fn main() {
    closure_capturing_borrow();
}
