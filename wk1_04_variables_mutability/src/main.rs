// we can specify constants
// we always have to specify the type of a constant
const MAX_POINTS: u32 = 100_000;

fn main() {
    // printing
    print!("Hello, world!\n");
    println!("Hello, world!");

    // variables are statically typed so types cannot change
    // often, types can be inferred by the compiler
    let x = 42;
    println!("x = {}", x);

    // variables are immutable by default
    // the following line will not compile
    // x = 13;

    // we can rebind a new value (of possibly different datatype) to x
    let x = 'x';
    println!("x = {}", x);

    // variables can be made mutable with the mut keyword
    let mut y = 13;
    y = y * 2;
    {
        // variables can be shadowed
        let y = y * 2;
        println!("y in braces = {}", y);
    }
    println!("y = {}", y);

    println!("MAX_POINTS = {}", MAX_POINTS);
}
