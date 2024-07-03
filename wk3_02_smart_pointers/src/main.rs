#![allow(unused)]

fn main() {
    /*
       any type T is stored on the stack
       T is owned
       but ONLY IF it has a known size at compile time
    */
    let x: &[i32] = &[1, 2, 3, 4, 5];
    // let y: [i32] = *x; // error: the size for values of type `[i32]` cannot be known at compilation time

    /*
        one solution: Box<T> (like unique_ptr in C++)
        Box<T> is a smart pointer to a heap-allocated value of type T
        Box<T> is stored on the stack, but the value of type T is stored on the heap
        the implication is if you own Box<T>, you own the value of type T

        once Box<T> goes out of scope, the value of type T is deallocated

        big limitation is performance
    */
    let y: Box<[i32]>;

    /*
        another solution: Rc<T> (like shared_ptr in C++)
        Rc<T> is a reference-counted smart pointer to a heap-allocated value of type T
        Rc<T> is stored on the stack, but the value of type T is stored on the heap
        the implication is if you own Rc<T>, you own the value of type T

        Rc<T> is a reference-counted smart pointer, so it keeps track of how many references there are to the value of type T
        when the last reference to the value of type T is dropped, the value of type T is deallocated

        Rc<T> is a good solution for when you need to share ownership of a value of type T
        but it is not a good solution for when you need to mutate the value of type T

        also cycles are an issue with Rc<T>: if you have two Rc<T> values that refer to each other, they will never be freed
    */
    use std::rc::Rc;

    let longer_lifetime;

    {
        let reference_counter: Rc<String> = Rc::new(String::from("hello"));
        longer_lifetime = reference_counter.clone();
    }

    println!("{}", longer_lifetime);

    /*
        RefCell<T> is a smart pointer that allows you to mutate the value of type T that it points to

        a bit of an anti-pattern -> does runtime checks instead and will panic if you violate the rules
    */
    use std::cell::RefCell;

    let value: RefCell<i32> = RefCell::new(5);
    *value.borrow_mut() += 1;
}
