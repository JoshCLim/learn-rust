use std::io;

use rand::Rng;

fn main() {
    // print
    println!("Guess the number!");

    // declare a variable -- variable is immutable by default
    // "secret_number" is a "binding"
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // can also explicitly state types
    // let secrete_number: i32 = 42;

    // "loop" is equivalent to "while true"
    loop {
        // "mut" keyword specifies that a variable is mutable
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .unwrap();

        // "trim" removes whitespace at start + end
        let number = input.trim().parse::<i32>().unwrap();

        match number.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
