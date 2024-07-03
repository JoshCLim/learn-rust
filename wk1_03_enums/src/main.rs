#![allow(unused)]

// think of it as a sum type (like in haskell)
// we can also attach data to each variant
enum COMP6991Member {
    Student {
        zid: u32,
        name: String,
    },
    Teacher {
        name: String,
    },
    Admin {
        name: String,
        hours: u32,
    },
    Lecturer {
        name: String,
        hours: u32,
        lecture_count: u32,
    },
    Convenor {
        name: String,
    },
}

enum Option<T> {
    Some(T),
    None,
}

fn print6991(member: COMP6991Member) {
    match member {
        COMP6991Member::Student { zid, name } => println!("student: {} {}", zid, name),
        COMP6991Member::Teacher { name } => println!("teacher: {}", name),
        COMP6991Member::Admin { name, hours } => println!("admin: {} {}", name, hours),
        COMP6991Member::Lecturer { name, hours, lecture_count } => {
            println!("lecturer: {} {} {}", name, hours, lecture_count)
        }
        COMP6991Member::Convenor { name } => println!("convenor: {}", name),
    }
}

fn main() {
    let student = COMP6991Member::Student { zid: 1234567, name: "John Smith".to_string() };
    let lecturer = COMP6991Member::Lecturer {
        name: "Jane Doe".to_string(),
        hours: 10,
        lecture_count: 2,
    };
    let convenor = COMP6991Member::Convenor {
        name: "Jane Doe".to_string(),
    };

    print6991(student);
    print6991(lecturer);
    print6991(convenor);

    let a = Option::Some(5);
    let b: Option<i32> = Option::None;
    match a {
        Option::Some(x) => println!("x: {}", x),
        Option::None => println!("None"),
    }
    match b {
        Option::Some(x) => println!("x: {}", x),
        Option::None => println!("None"),
    }
}
