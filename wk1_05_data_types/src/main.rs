type _Point = (i32, i32); // type alias

fn main() {
    println!("Hello, world!");
}

// * INTEGERS * //
fn _int() {
    // i32 is default int -- 32 bits
    let _a: i32 = 45; 
    
    // u32 is unsigned int -- number is never negative
    // can specify that a number is never negative in our type system
    let _a: u32 = 18; 

    // other int types
    let _a: i8 = 1; // 8 bits
    let _a: i16 = 1; // 16 bits
    let _a: i64 = 1; // 64 bits
    let _a: i128 = 1; // 128 bits
    // also u8, u16, u64, u128
}

// * FLOATS * //
fn _float() {
    // floating point types
    let _a: f32 = 1.0; // 32 bits (float)
    let _a: f64 = 1.0; // 64 bits (double)
}

fn _char() {
    // char -- Unicode UTF-32 characters (so not just ASCII)
    let _a: char = 'a'; 
}

fn _bool() {
    // boolean values
    let _a: bool = false;
}

// * TUPLES * //
fn _tuple() {
    // tuples => represent groups of values
    let _a: (i32, f64, char) = (500, 6.4, 'a');
    let (_b, _c, _d) = _a; // we can destructure tuples 
    let (_f, _, _g) = _a; // we can destructure tuples and ignore some values
    let (_e, ..) = _a; // we can destructure tuples and ignore the rest
    println!("{} {} {}", _a.0, _a.1, _a.2); // we can also access tuples by index
}

// * ARRAYS (FIXED-SIZE) * //
fn _arrays() {
    // arrays => represent groups of values
    let _a: [i32; 5] = [0, 1, 2, 3, 4];
    let _b: i32 = _a[0]; // we can access arrays by index
}

// * STRUCTS * //
fn _structs() {
    // structs => represent groups of values
    struct Point {
        _x: i32,
        _y: i32,
    }
    let _origin: Point = Point { _x: 0, _y: 0 };
    println!("{} {}", _origin._x, _origin._y); // we can access struct fields by name
    let Point { _x: _a, _y: _b } = _origin; // we can destructure structs

    // note: immutability is very pervasive... we cannot change the values of the struct (often unlike other languages)
    // _origin._x = 1; // this will not compile
}

// * ENUMS * //
fn _enums() {
    // enums => represent a fixed set of possibilities
    enum _Direction {
        _Up,
        _Down,
        _Left,
        _Right,
    }
    let _a: _Direction = _Direction::_Up;
    match _a {
        _Direction::_Up => println!("Up"),
        _Direction::_Down => println!("Down"),
        _Direction::_Left => println!("Left"),
        _Direction::_Right => println!("Right"),
    }
    // enums can also have data associated with them
                            // equivalent Haskell code:
    enum _Vehicle {         // data Vehicle = 
        Car(String),        //   Car String | 
        Truck(String),      //   Truck String |
        Bicycle,            //   Bicycle
    }
    
    enum _FancyVehicle {
        _Car { make: String, model: String },
        _Train { capacity: u32 },
        _Motorbike { make: String, model: String, year: u32 },
    }
    let _a: _FancyVehicle = _FancyVehicle::_Car { make: String::from("Toyota"), model: String::from("Prius") };
    match _a {
        _FancyVehicle::_Car { make, model } => println!("{} {}", make, model),
        _FancyVehicle::_Train { capacity } => println!("{}", capacity),
        _FancyVehicle::_Motorbike { make, model, year } => println!("{} {} {}", make, model, year),
    }
}

// * UNIT TYPE * //
fn _unit() {
    // unit type => equivalent to an empty value
    let _a: () = ();

    // the unit type only has one possible value: ()
    // we can use this type to represent nothing
}

// * FUNCTIONS * //
// function that takes an i32
fn _foo(x: i32) {
    println!("{}", x);
}

// function that takes a tuple
fn _bar(x: (i32,)) {
    println!("{}", x.0);
}

// function that takes an array
fn _baz(x: [i32; 5]) {
    println!("{}", x[0]);
}
