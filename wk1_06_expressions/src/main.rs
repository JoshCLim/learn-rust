fn main() {
    /*
        Rust is an 'expression-oriented' language.

        Expressions return a value, and statements do not.
        An expression-statement is an expression that is used as a statement (i.e., it does something but it also evaluates to a value)

        Expressions are more powerful than statements because you can use them in more places -- all expressions can be used as statements;

        any time you have a block {} that ends with an expression, you can omit the semicolon and 'block' will evaluate to that expression.

        e.g. 'if's are expressions
            let x = if flag {
                1
            } else {
                2
            };
     */

    let _x = 4; // this is a statement

    42; // this is an expression (being uselessly used as a statement)

    let _y = { // this is an expression
        let x = 3;
        x + 1 // no semicolon here, otherwise it becomes a statement
    };

    println!("{}", if _x == 4 { "yes" } else { "no" });
}

// if expression used as a ternary operator
fn _if_expression(flag: bool) -> i32 {
    // bad code
    let x: i32;

    if flag {
        x = 1;
    } else {
        x = 2;
    }

    println!("x = {}", x);


    // better code
    let x = if flag {
        1
    } else {
        2
    };

    println!("x = {}", x);
    x // same as "return x;"
}

// if the expression does not return a value, it is assigned the unit type
fn _unit_type() {
    // The unit type is written as (), and has only one value, also written ().
    // A unit type is useful when you need to represent the concept of "nothing" in situations where Rust requires a type.
    let _x: () = if true {
        println!("true");
    } else {
        println!("false");
    };
}

// nested if statements (note: return types must be the same)
fn _nested_if() -> i32 {
    let x = 5;

    if x == 5 {
        if x == 6 {
            1
        } else {
            2
        }
    } else {
        3
    }
}

/**
 * function to find the log base 2 of a number
 * the 'return None' statement has the 'never' type
 */
fn _log_2(num: f32) -> Option<f32> {
    let log: f32 = if num == 0.0 {
        return None;
    } else {
        num.log2()
    };

    Some(log)
}

fn _infinite_loop() {
    loop {
        println!("infinite loop");
    }
}

fn _break_with_value() {
    let mut x = 5;

    let y = loop {
        x += 1;

        if x == 10 {
            break x * 2;
        }
    };

    println!("y = {}", y);
}

fn _while_loop() {
    let mut x = 5;

    while x != 0 {
        println!("x = {}", x);
        x -= 1;
    }
}

fn _for_loop() {
    for x in 0..5 {
        println!("x = {}", x);
    }

    let x = [1, 2, 3, 4, 5];
    for elem in x {
        if elem == 4 {
            continue;
        }

        println!("elem = {}", elem);
    }
}

// matches need to be exhaustive
fn _match() {
    let x: u32 = 5;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        // use | to separate multiple options
        6 | 7 | 8 => println!("big number"),
        // _ is used as wildcard catch-all 
        _ => println!("something bigger"),
    }

    let y: Option<i32> = Some(5);
    match y {
        Some(v) => println!("v = {}", v),
        None => println!("none"),
    }

    let z: Result<i32, i32> = Ok(3);
    match z {
        Ok(v) => println!("v = {}", v),
        Err(e) => eprintln!("e = {}", e),
    }
}
