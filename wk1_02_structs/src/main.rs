struct Student {
    zid: u32,
    name: String,
    age: u8, 
}

fn main() {
    println!("Hello, world!");

    let student1 = Student {
        zid: 1234567,
        name: String::from("John Smith"),
        age: 20,
    };

    println!("Student 1: {} {} {}", student1.zid, student1.name, student1.age);
}
