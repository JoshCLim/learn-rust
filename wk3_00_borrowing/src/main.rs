#![allow(unused)]

fn main() {
    // move semantics examples
    move_semantics();

    // copy semantics examples
    copy_semantics();

    // make a type copyable
    make_copy();

    // the clone method
    string_clone();

    // this function demonstrates cascading ownership in Rust
    cascading_ownership_structs_enums(); 

    // this function demonstrates the pain of ownership in Rust
    ownership_pain(); 

    // issues that can arise from shared mutable state
    dangers_of_shared_mutable_state();

    // shared xor mutable
    shared_xor_mutable();

    // borrowing
    borrowing();

    // [aside] dangling references are banned in Rust
    dangling_references();

    // slices
    slices();

    // [aside] iterators and borrowing
    iters();
}

struct Student {
    zid: u32,
    name: String,
    wam: Option<f64>,
}

fn move_semantics() {
    // [EXAMPLE: move semantics with variable assignment or calling a function]
    fn print_student(student: Student) {
        println!("Student: {} ({})", student.name, student.zid);
        match student.wam {
            Some(wam) => println!("WAM: {}", wam),
            None => println!("WAM: N/A"),
        }
    }

    let student = Student {
        zid: 1234567,
        name: String::from("John Doe"),
        wam: Some(75.0),
    };

    let other_student = student;

    // println!("{}", student.zid); // error: value used here after move to other_student

    print_student(other_student);

    // print_student(other_student); // error: value was moved on the first call to print_student
}

fn copy_semantics() {
    // Rust also has copy types -- these are types that are copied when assigned to a new variable instead of moved
    let x = 42;
    let y = x;

    println!("x: {}, y: {}", x, y); // this is fine because i32 is a copy type
}

fn make_copy() {
    // we can make a struct copy only if all its fields are copy types
    #[derive(Copy, Clone)] // this is a derive attribute that implements the Copy and Clone traits for the struct
    struct CopyableFoo {
        x: i32,
        y: bool
    }

    let foo = CopyableFoo { x: 42, y: true };
    let bar = foo;

    println!("foo.x: {}, foo.y: {}", foo.x, foo.y); // this is fine because CopyableFoo is a copy type
    println!("bar.x: {}, bar.y: {}", bar.x, bar.y); // this is fine because CopyableFoo is a copy type
}

fn make_move() {
    // we can make a copy type have move semantics by wrapping it in a struct
    struct MoveI32(i32);

    let x = MoveI32(42);
    let y = x;

    // println!("x: {}", x.0); // error: value used here after move to y
    println!("y: {}", y.0);

    // we can implement the Clone trait for MoveI32 to make it copyable
    impl Clone for MoveI32 {
        fn clone(&self) -> Self {
            MoveI32(self.0)
        }
    }
    // now we can clone MoveI32
    let x = MoveI32(42);
    let y = x.clone();
}

fn string_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1: {}, s2: {}", s1, s2); // this is fine because we have cloned the string
    // think about copy as just implicitly calling clone, except copy will only ever copy the bits in memory (and never do other stuff like open files, etc.)
}

fn cascading_ownership_structs_enums() {
    let name = String::from("John Doe");

    let student = Student {
        zid: 1234567,
        name,
        wam: Some(75.0),
    };

    // println!("{}", name); // error: value used here after move to student
}

fn ownership_pain() {
    fn inflate_wam(mut student: Student) -> Student {
        match student.wam {
            Some(wam) => student.wam = Some(wam + 5.0),
            None => (),
        }

        student
    }

    let student = Student {
        zid: 1234567,
        name: String::from("John Doe"),
        wam: Some(75.0),
    };

    // inflate_wam(student); // error: value was moved here so can't use it later
    let student = inflate_wam(student); // this is fine because we are reassigning the student variable (i.e., ownership is transferred back to the caller via the return value)
    // but this sucks because we have to reassign... what if we want to mutate the student in place?

    match student.wam {
        Some(wam) => println!("WAM: {}", wam), // error: value used here after move to inflate_wam
        None => println!("WAM: N/A"),
    }
}

fn dangers_of_shared_mutable_state() {
    /* 
    if we have shared mutable state, we can run into lots of issues:
    
    - race conditions
        two threads trying to read and write to the same memory location -> leads to undefined behaviour
        write-write data race
        read-write data race
        note: multiple threads is ok if all threads are only reading (i.e., immutable state)
    
    - iterator invalidation 
        (e.g., in C++ if you edit a vector while iterating over it)
    
    any time someone is writing to memory that someone else is reading/writing from, there is danger danger
    */
}

fn shared_xor_mutable() {
    /*
    xor = one or the other but not both

    so basic idea: memory can be shared or mutable but not both
    note: this is a temporal -> I can switch between shared and mutable but not at the same time

    Rust must determine at compile time that all memory is either shared or mutable but not both

    [aside] at the moment, the Rust compiler is occassionally too conservative and will not allow you to do things that are safe
    */
}

fn borrowing() {
    /*  ---------------------------------
        [SUMMARY: borrowing]
        borrowing is a way to share memory without giving up ownership

        two types:
        - shared borrow: & (reference)
        - exclusive borrow: &mut (mutable reference)

        so three types in total:
        - owned: T                  (ownership)
        - shared borrow: &T         (read-only)
        - exclusive borrow: &mut T  (read-write)
        --------------------------------- */

    /*
    [SUMMARY: shared borrows]
    - can be shared infinitely many times 
    - ALL shared borrows use copy semantics
    - very cheap to pass to functions -- just a pointer is passed

    - but cannot mutate the memory: no writes allowed
    - cannot take any exclusively borrowed references while shared borrows are in scope
    */

    // [EXAMPLE: shared borrow]
    // note: we are guaranteed that no one else can mutate the memory while we have a shared borrow
    fn string_chars_len(string: &String) -> usize {
        // string.push('1'); // error: cannot borrow `*string` as mutable, as it is behind a `&` reference
        string.chars().count()
    }

    let mut s = String::from("foo");
    let len = string_chars_len(&s);
    s.push_str("bar");


    /*
    [SUMMARY: exclusive borrows]
    exclusive borrows:
    - can mutate data without taking ownership
    - very cheap to pass to functions -- just a pointer is passed

    - cannot take any further exclusive references during lifetime
    - cannot take any shared references during lifetime
    */

    // [EXAMPLE: exclusive borrow]
    // adds "!!!" to the end of the string and makes it uppercase
    fn emphasize(string: &mut String) { // use an exclusive borrow
        string.make_ascii_uppercase();
        string.push_str("!!!");
    }

    let mut s2 = String::from("foo");
    emphasize(&mut s2);
    println!("{}", s2);

    let s3 = String::from("hi");
    // emphasize(&mut s3); // error: cannot borrow non mutable reference

    let mut s4 = String::from("boo");
    // scope of this excl_borrow ends two lines later
    let mut excl_borrow_1 = &mut s4;
    // let mut excl_borrow_2 = &mut s4; // error: cannot create multiple exclusive borrows
    emphasize(excl_borrow_1);
}

fn dangling_references() {
    /*
    bad C code:
    List ListNew() {
        struct list l;

        return &l; // stack-use-after-free
    }
    */

    // the Rust compiler avoids this

    // struct LinkedList { }
    // 
    // fn list_new() -> &LinkedList { // ! error
    //     let list = LinkedList { };
    //     &list
    // }
}

fn slices() {
    let x = [1,2,3,4,5]; // on the stack, fixed size
    let y = vec![1,2,3,4,5]; // on the heap, can change size

    // but if I borrow an array (&[i32; 5]) or a vector (&vec<i32>) note that
    // since I cannot modify the vector, it's equivalent to an array...

    // syntax for a slice is [T]
    // vector Vec<i32> -> [i32]
    // array [i32; 5] -> [i32]

    // [EXAMPLE: slice]
    // &[T] is a slice of T
    // it carries a pointer to the start of the slice and the length of the slice
    fn sum(slice: &[i32]) -> i32 {
        slice.iter().sum()
    }

    let x = [1,2,3,4,5];
    let y = vec![1,2,3,4,5];
    let sum_x = sum(&x);
    let sum_y = sum(&y);
    println!("sum_x: {}, sum_y: {}", sum_x, sum_y);

    // slices are very useful for passing around parts of arrays and vectors
    // without having to copy the data
    let slice = &x[1..3]; // slice of x from index 1 to 3 (exclusive)
    sum(slice);

    // [EXAMPLE: mutable slice]
    fn zero_out(slice: &mut [i32]) {
        for elem in slice.iter_mut() {
            *elem = 0;
        }
    }
    let mut x = [1,2,3,4,5];
    zero_out(&mut x[1..3]);

    // [EXAMPLE: string slice]
    // string slice is denoted by &str
    let s = String::from("hello");
    let slice: &str = &s[1..3]; // slice of s from index 1 to 3 (exclusive)
    println!("{}", slice);

    // [EXAMPLE: empty and full slice]
    let empty = &x[0..0]; // empty slice
    let full = &x[..]; // slice of x from index 0 to end

    // [EXAMPLE: leading zero can be omitted]
    let slice = &x[..3]; // slice of x from index 0 to 3 (exclusive)
}

fn iters() {
    let x = vec![1,2,3,4,5];

    // [EXAMPLE: into_iter consumes the vector, but the elements are owned]
    // into_iter(self)
    for a in x.into_iter() {
        println!("{}", a);
    }
    // x; // error: value used here after move

    // [EXAMPLE: iter borrows the vector, but the elements are borrowed]
    // iter(&self)
    let x = vec![1,2,3,4,5];
    for a in x.iter() {
        println!("{}", a);
    }
    x; // this is fine because we only borrowed the vector

    // [EXAMPLE: iter_mut borrows the vector mutably, and the elements are borrowed mutably]
    // iter_mut(&mut self)
    let mut x = vec![1,2,3,4,5];
    for a in x.iter_mut() {
        *a += 1;
    }
    x; // this is fine because we only borrowed the vector mutably
}
