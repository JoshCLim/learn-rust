fn main() {
    // memory_safety
    importance_of_memory_safety();

    // unsafe keyword
    unsafe_rust();

    // example: String::from_utf8_unchecked

    // checking unsafe code
    checking_unsafe_code();
}

fn importance_of_memory_safety() {
    // safe Rust is free from null pointer dereferencing, buffer overflows, and other memory safety bugs
    // -- most languages are

    // with C and Cpp etc., there is a lot of burden on the programmer to ensure memory safety
    // and lots of security issues arise from memory safety bugs

    // ideal world: a program will compile iff it is correct
    // Rust compiles most correct programs, and does not compile all incorrect programs and some correct programs

    // the realm of systems programming (low-level) -> we need low-level access to pointers, memory, etc.

    // Rust uses abstractions to maintain memory safety (e.g. the Vec<T> type)
}

fn unsafe_rust() {
    // features of unsafe Rust:
    // - unsafe keyword
    // - raw pointers: *const T, *mut T
    // - module std::ptr
    // - module std::alloc
    // - module std::ffi
    // - PhantomData

    // safe Rust is a subset of unsafe Rust

    // advantages of safe Rust:
    // - memory safety by default
    // - no undefined behaviour -- if a program leads to undefined behaviour, it won't compile (unless there is a compiler bug)
    // - relatively low complexity (compared to unsafe Rust)
    // - opportunity for quality diagnostics
    // - opportunity for optimisation -> compiler can make assumptions about the code and has more freedom to perform optimisations

    // advantages of unsafe Rust:
    // - low-level control
    // - raw access to memory
    // - fine-grained access in pathalogical cases for optimisation

    // disadvantages of safe Rust:
    // - isn't very flexible

    // disadvantages of unsafe Rust:
    // - undefined behaviour
    // - complexity
    // - memory unsafety

    // idea of safe abstractions hiding unsafe code:
    // - Rust consists of safe abstractions that hide unsafe code -> so all the inputs that go into the box will be safe and produce safe output, but the box itself may contain unsafe code
    // - unsafe code can do these extra things:
    //   1. dereference a raw pointer
    //   2. reading or writing a mutable or external static variable
    //   3. reading a field from a union
    //   4. calling an unsafe function
    //   5. implementing an unsafe trait
}

// example: String::from_utf8_unchecked -- an unsafe function
//
// has better performance than String::from_utf8 but the caller must ensure
// that the bytes are valid UTF-8
#[test]
fn from_utf8_unchecked() {
    let sparkle_heart = vec![240, 159, 146, 150];

    // safety: the caller must ensure that the bytes are valid UTF-8
    let sparke_heart = unsafe { String::from_utf8_unchecked(sparkle_heart) };

    assert_eq!("💖", sparke_heart);
}

fn checking_unsafe_code() {
    // tools to check unsafe code:
    // - sanitizers: tools that check for memory safety issues, but slow down runtimes
    //   e.g. AddressSanitizer, ThreadSanitizer, MemorySanitizer
    // - Miri: an interpreter for Rust's mid-level intermediate representation (MIR)
    //   ```rustup +nightly component add miri```
    //   ```cargo +nightly miri run```
    // - Loom: for testing concurrent code
    //   will run all possible interleavings of threads
}
