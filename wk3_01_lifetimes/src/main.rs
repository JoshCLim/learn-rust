#![allow(unused)]

fn main() {
    // [review] dangling reference
    dangling();

    // lifetimes
    lifetimes();

    // elision
    elision();

    // static lifetime
    static_lifetime();

    // struct and enum lifetimes
    struct_and_enum_lifetimes();
}

fn dangling() {
    // BAD: dangling reference
    /*
    let r: &i32; // lifetime of `r` starts here

    {
        let x = 5; // lifetime of `x` starts heres
        r = &x; // error: `x` does not live long enough
    } // lifetime of `x` ends here
    */

    // println!("r: {}", r); // lifetime of `r` ends here

    // OK:
    let x = 5; // lifetime of `x` starts here
    let r: &i32; // lifetime of `r` starts here
    r = &x;
    println!("r: {}", r);
    // lifetime of `r` ends here
    // lifetime of `x` ends here
}

fn lifetimes() {
    // lifetimes

    /* [EXAMPLE: will not compile due to ambiguity in return value's lifetime]
    fn longest(x: &str, y: &str) -> &str { // error: missing lifetime specifier
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    */

    // [EXAMPLE: specifying that the return value has the same lifetime as the input parameters]
    // if lifetime of one is longer than other, then the return value
    //   will have the same lifetime as the shorter one
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // more verbose (not as idiomatic) version
    fn longest2<'x, 'y, 'z>(x: &'x str, y: &'y str) -> &'z str
    where
        'x: 'z, // means, where 'x outlives 'z
        'y: 'z,
    {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // suppose we have two strings: one with a long lifetime and one with a short lifetime
    let long_lifetime_str = String::from("abcd"); // 'x
    {
        {
            let result: &str;
            {
                let short_lifetime_str = String::from("ah"); // 'y

                // note: we say
                //      'x : 'y
                // meaning that 'x outlives 'y

                result = longest(&long_lifetime_str, &short_lifetime_str);
                println!("The longest string is {}", result);
            }
            // println!("The longest string is {}", result); // error: `short_lifetime_str` does not live long enough

            // other code...
        }
        // other code...
    }
}

fn elision() {
    // elision: where it is possible to elide lifetimes (infer? the lifetimes)

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }
        &s[..]
    }

    // more verbose (worse) version
    fn first_word2<'a>(s: &'a str) -> &'a str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }
        &s[..]
    }

    let s = String::from("hello world");
    let word = first_word(&s);
    println!("The first word is: {}", word);
}

fn static_lifetime() {
    // static lifetime: lifetime of the entire program

    fn empty_string() -> &'static str {
        ""
    }
}

fn struct_and_enum_lifetimes() {
    // struct and enum lifetimes

    /* BAD: missing lifetime specifier for struct declaration
    struct StructWithBorrow {
        x: &i32, // error: missing lifetime specifier
    }

    enum EnumWithBorrow {
        X(&i32), // error: missing lifetime specifier
    }
    */

    // [EXAMPLE: struct with lifetime]
    struct StructWithBorrow<'a> {
        x: &'a i32,
    }

    // [EXAMPLE: enum with lifetime]
    enum EnumWithBorrow<'a> {
        X(&'a i32),
    }

    // [EXAMPLE: function returning a struct with a borrow]
    fn wrap_in_struct<'a>(x: &'a i32) -> StructWithBorrow<'a> {
        StructWithBorrow { x }
    }
}
