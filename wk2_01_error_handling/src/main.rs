/*
    canonical / idiomatic = using common conventions or style

    how do programming languages handle errors?
        
        1. sentinel values
        where a specific value is returned to indicate an error
        e.g. error codes, a function returns null or -1 if failure

        found in C, Go, Pascal

        2. unchecked exceptions
        where an error is thrown and the program looks for a handler
        if no handler is found, the program crashes
        (like try-catch blocks)

        found in Python, Java, Swift, Kotlin, C++, C#

        3. checked exceptions
        where an error is thrown and the program looks for a handler
        but if no handler is found, the program will not compile

        e.g. Java

        4. first-class sum types
        where a type is used to represent either success or failure
        
        e.g. Rust, Haskell, Swift
*/

enum GetInputError {
    InputFailed,
    InputEmpty,
    InputWasNotNumber,
}

// allows us to use the ? operator
impl From<std::io::Error> for GetInputError {
    fn from(_error: std::io::Error) -> Self { // Self refers to GetInputError
        GetInputError::InputFailed
    }
}

impl From<std::num::ParseIntError> for GetInputError {
    fn from(_error: std::num::ParseIntError) -> Self {
        GetInputError::InputWasNotNumber
    }
}

fn get_input() -> Result<i32, GetInputError> {
    let mut input = String::new();
    // let result = 
    std::io::stdin().read_line(&mut input)?;

    // the ? at the end of the line just returns the error (based on the above impl statements)

    // match result {
    //     Ok(_) => {},
    //     Err(_) => return Err(GetInputError::InputFailed)
    // };

    if input.is_empty() {
        return Err(GetInputError::InputEmpty)
    }

    let num = input.trim().parse()?;

    // or to avoid the conversions above,
    // .map_err(|_error| GetInputError::InputWasNotNumber)

    // match num {
    //     Ok(num) => Ok(num),
    //     Err(_) => Err(GetInputError::InputWasNotNumber)
    // }

    Ok(num)
}

fn _something_goes_horribly_wrong() {
    // panic! macro is used to throw an uncatchable error?
    panic!("This is a panic!");
}

fn main() {
    let input = get_input();
    match input {
        Ok(num) => println!("Input was {}", num),
        Err(err) => match err {
            GetInputError::InputFailed => println!("Failed to get input"),
            GetInputError::InputWasNotNumber => println!("Input was not a number"),
            GetInputError::InputEmpty => println!("Input was empty"),
        }
    }
}