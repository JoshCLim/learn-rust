fn main() {
    // this is erroneous code
    let x = String::from("hello");
    let y = x;

    println!("{y}");
    // println!("{x}"); // ! ERROR

    /*
        why? 

        String is not a primitive type -- it is malloced on the heap.

        in a language like C, if we did, 
            char *x = "hello";
            char *y = x; // copy the reference
        y would be a pointer to the same memory location as x

        in a language like C++, if we did,
            string x = "hello";
            string y = x; // copy assignment
        y would be a copy of x (i.e., a new string with the same value as x)

        in Rust, if we did,
            let x = String::from("hello");
            let y = x; // move assignment
        y would be a move of x (i.e., x would be invalidated and y would be the 
            new owner of the memory location)

        helps us to avoid double free errors, etc. whilst maintaining performance
        (i.e., no deep copies)
        this is accomplished by ensuring there is only one owner of a given memory

        also, relevant to file descriptors, etc. (i.e., resources that are not 
            memory) -- e.g., if we had a file descriptor, we would not want to
            close it twice, etc.

        ownership is determined at compile time -- no performance impact at runtime
    */

    // instead, we can get copy semantics via .clone()
    let x = String::from("hello");
    let y = x.clone();

    println!("{y}");
    println!("{x}"); // OK

    // note that for 'copy' types, it uses copy semantics instead of move semantics
    let x = 5;
    let y = x;
    println!("{y}");
    println!("{x}"); // OK

    // tuples are also 'copy' types if all of their elements are 'copy' types
    let x = (1, 2, 3);
    let y = x;
    println!("{}", y.0);
    println!("{}", x.0); // OK

    let x = (1, String::from("hello"));
    let y = x;
    println!("{}", y.0);
    // println!("{}", x.0); // ! ERROR

    // structs are not copy types
    struct Foo {
        fst: i32,
        snd: i32,
    }
    let x = Foo { fst: 1, snd: 2 };
    let y = x;
    println!("{} {}", y.fst, y.snd);
    // println!("{} {}", x.fst, x.snd); // ! ERROR

    // we can get copy semantics for structs by implementing the Copy trait
    #[derive(Copy, Clone)]
    struct Foo2 {
        fst: i32,
        snd: i32,
    }
    let x = Foo2 { fst: 1, snd: 2 };
    let y = x;
    println!("{} {}", y.fst, y.snd);
    println!("{} {}", x.fst, x.snd); // OK

    // enums are the same as structs

    // arrays are never copy types
}

fn _foo(x: String) {
    println!("{}", x);
}

fn _bar() {
    let x = String::from("hello");
    _foo(x); // transfer ownership of 'x' to foo
    // println!("{}", x); // ! ERROR
}

fn _foo2(x: String) -> String {
    println!("{}", x);
    return x;
}

fn _bar2() {
    let x = String::from("hello");
    let x = _foo2(x); // transfer ownership of 'x' to foo
    println!("{}", x); 
    drop(x); // drop = Rust's version of free -- will be done implicitly but can be done explicitly
}
