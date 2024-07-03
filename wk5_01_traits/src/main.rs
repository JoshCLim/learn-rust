#![allow(unused)]

use std::ops::Add;

fn main() {
    std_library_traits();

    using_traits_via_impl_derive();

    ad_hoc_polymorphism();

    static_dynamic_dispatch();
}

/// traits (ad-hoc polymorphism) -- like Java interfaces?

/// std library traits:
///
/// # general traits:
/// - `Default`
/// - `Clone`
/// - `Copy`
///
/// # formatting traits:
/// - `Debug`
///
/// - `Display` -- similar to `ToString` but uses a `Formatter`
///                so the display function can be customized instead of just
///                returning a string (e.g. printing out as you go instead of
///                returning a very long string)
///
/// - `ToString` -- can convert to `String`
///
/// # error traits:
/// - `Error`
///`
/// # comparison traits:
/// - `PartialEq`
/// - `PartialOrd`
///
/// # iterator traits:
/// - `Iterator`
/// - `IntoIterator`
/// - `FromIterator` -- can be used to create a collection from an iterator (using `.collect(...)`)
fn std_library_traits() {}

fn using_traits_via_impl_derive() {
    // the `#[derive(SOME_TRAIT)]` attribute is a shorthand for implementing that trait
    // [EXAMPLE: derive(Clone)]
    #[derive(Clone)]
    struct Point {
        x: i32,
        y: i32,
    }
    // can use `cargo expand` to see the expanded code for the `derive` attribute

    // or we can implement the trait manually
    struct Point2 {
        x: i32,
        y: i32,
        z: String,
    }
    impl Clone for Point2 {
        fn clone(&self) -> Self {
            Self {
                x: self.x,
                y: self.y,
                z: self.z.clone(),
            }
        }
    }
    impl Add for Point2 {
        type Output = Point2;

        fn add(self, other: Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z,
            }
        }
    }

    // impl with generics
    struct Point3<T> {
        x: T,
        y: T,
    }
    impl<T> Add for Point3<T>
    where
        T: Add<Output = T>,
    {
        type Output = Self;

        fn add(self, other: Self) -> Self::Output {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
}

mod stuff {
    pub trait Animal {
        fn name(&self) -> String;
        fn age(&self) -> u32;
        fn speak(&self) -> String;

        // prevents the creation of trait object (dynamic dispatch) -> since the function has a generic type parameter, it can't be turned into a trait object
        fn say_hello_to<A>(&self, animal: &A) -> String
        where
            A: Animal,
            // Self: Sized, // this is an escape hatch to allow the creation of a trait object -> and it will just stop this function from being called on a dyn Animal
        {
            format!("{} says {} to {}", self.name(), self.speak(), animal.name())
        }
    }

    pub struct Sheep {
        pub name: String,
        pub age: u32,
        pub at_party: bool,
    }
    pub struct Cow {
        pub name: String,
        pub age: u32,
        pub angry: bool,
    }

    impl Animal for Sheep {
        fn name(&self) -> String {
            self.name.clone()
        }

        fn age(&self) -> u32 {
            self.age
        }

        fn speak(&self) -> String {
            if self.at_party {
                String::from("Ba tss Ba tss Ba tss")
            } else {
                String::from("baAaAaA")
            }
        }

        fn say_hello_to<A>(&self, animal: &A) -> String
        where
            A: Animal,
        {
            if self.at_party {
                format!("how's the baAaArty, {}?", animal.name())
            } else {
                format!("baAaA to you, {}", animal.name())
            }
        }
    }
    impl Animal for Cow {
        fn name(&self) -> String {
            self.name.clone()
        }

        fn age(&self) -> u32 {
            self.age
        }

        fn speak(&self) -> String {
            if self.angry {
                String::from("MOOOOO")
            } else {
                String::from("moo")
            }
        }

        fn say_hello_to<A>(&self, animal: &A) -> String
        where
            A: Animal,
        {
            if self.angry {
                format!("MOOOOO you {}", animal.name())
            } else {
                format!("moouwu, {}", animal.name())
            }
        }
    }
}

fn ad_hoc_polymorphism() {
    use stuff::*;

    let shaun = Sheep {
        name: String::from("Shaun"),
        age: 4,
        at_party: true,
    };

    let betsie = Cow {
        name: String::from("Betsie"),
        age: 6,
        angry: false,
    };

    // note: in order to call the methods on the trait, the trait must be in scope

    println!("{}", shaun.speak());
    println!("{}", betsie.speak());
    println!("{}", shaun.say_hello_to(&betsie));
    println!("{}", betsie.say_hello_to(&shaun));

    struct BestFriends<A1, A2> {
        a1: A1,
        a2: A2,
    }
    // could also put the restriction on the struct instead of the impl -- opinion thing
    impl<A1: Animal, A2: Animal> BestFriends<A1, A2> {
        fn both_greet(&self) -> String {
            format!(
                "{}\n{}",
                self.a1.say_hello_to(&self.a2),
                self.a2.say_hello_to(&self.a1)
            )
        }
    }

    let besties = BestFriends {
        a1: shaun,
        a2: betsie,
    };
    println!("{}", besties.both_greet());
}

mod stuff_dyn {
    pub trait AnimalDyn {
        fn name(&self) -> String;
        fn age(&self) -> u32;
        fn speak(&self) -> String;

        fn say_hello_to(&self, animal: &dyn AnimalDyn) -> String;
    }

    pub struct Sheep {
        pub name: String,
        pub age: u32,
        pub at_party: bool,
    }
    pub struct Cow {
        pub name: String,
        pub age: u32,
        pub angry: bool,
    }

    impl AnimalDyn for Sheep {
        fn name(&self) -> String {
            self.name.clone()
        }

        fn age(&self) -> u32 {
            self.age
        }

        fn speak(&self) -> String {
            if self.at_party {
                String::from("Ba tss Ba tss Ba tss")
            } else {
                String::from("baAaAaA")
            }
        }

        fn say_hello_to(&self, animal: &dyn AnimalDyn) -> String {
            if self.at_party {
                format!("how's the baAaArty, {}?", animal.name())
            } else {
                format!("baAaA to you, {}", animal.name())
            }
        }
    }

    impl AnimalDyn for Cow {
        fn name(&self) -> String {
            self.name.clone()
        }

        fn age(&self) -> u32 {
            self.age
        }

        fn speak(&self) -> String {
            if self.angry {
                String::from("MOOOOO")
            } else {
                String::from("moo")
            }
        }

        fn say_hello_to(&self, animal: &dyn AnimalDyn) -> String {
            if self.angry {
                format!("MOOOOO you {}", animal.name())
            } else {
                format!("moouwu, {}", animal.name())
            }
        }
    }
}

fn static_dynamic_dispatch() {
    use stuff;
    use stuff_dyn;

    // what if I had a Vec of animals?
    // problem: I can't mix and match different types of animals in a Vec
    fn all_animals_greet<A>(animals: Vec<A>)
    where
        A: stuff::Animal,
    {
        for animal in animals {
            println!("{}", animal.speak());
        }
    }
    let animals = vec![
        stuff::Sheep {
            name: String::from("Shaun"),
            age: 4,
            at_party: true,
        },
        stuff::Sheep {
            name: String::from("Alex"),
            age: 3,
            at_party: false,
        },
        stuff::Sheep {
            name: String::from("Miguel"),
            age: 6,
            at_party: false,
        },
    ];
    all_animals_greet(animals); // by monomorphization, this will cause the function to be generated for the type Sheep only

    // [ERROR]
    // let various_animals: Vec<Animal> = vec![
    //     Sheep {
    //         name: String::from("Shaun"),
    //         age: 4,
    //         at_party: true,
    //     },
    //     Cow {
    //         name: String::from("Betsie"),
    //         age: 6,
    //         angry: false,
    //     },
    // ];

    // what if I had another animal struct with a different size
    struct Crab {
        age: u32,
    }
    impl stuff_dyn::AnimalDyn for Crab {
        fn name(&self) -> String {
            String::from("Crab")
        }

        fn age(&self) -> u32 {
            self.age
        }

        fn speak(&self) -> String {
            String::from("click click click")
        }

        fn say_hello_to(&self, animal: &dyn stuff_dyn::AnimalDyn) -> String {
            format!("click click click to you, {}", animal.name())
        }
    }

    // we can use Box since it will always be the same size, also use dyn

    fn all_animals_greet_dyn(animals: Vec<Box<dyn stuff_dyn::AnimalDyn>>) {
        for animal in animals {
            println!("{}", animal.speak());
        }
    }

    let animals_dyn: Vec<Box<dyn stuff_dyn::AnimalDyn>> = vec![
        Box::new(stuff_dyn::Sheep {
            name: String::from("Shaun"),
            age: 4,
            at_party: true,
        }),
        Box::new(stuff_dyn::Cow {
            name: String::from("Betsie"),
            age: 6,
            angry: false,
        }),
    ];
    all_animals_greet_dyn(animals_dyn);
}

fn implementing_iterator() {
    // [EXAMPLE: Fibonacci]
    struct Fibonacci {
        prev: u32,
        curr: u32,
    }

    fn fib() -> Fibonacci {
        Fibonacci { prev: 0, curr: 1 }
    }

    impl Iterator for Fibonacci {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            let new = self.prev + self.curr;
            self.prev = self.curr;
            self.curr = new;
            Some(self.prev)
        }
    }

    for i in fib().take(10) {
        println!("{}", i);
    }

    // [EXAMPLE: vec iterator]
    struct VecIter<'a, T> {
        vec: &'a Vec<T>,
        index: usize,
    }
    fn my_vec_iter<T>(vec: &Vec<T>) -> VecIter<T> {
        VecIter { vec, index: 0 }
    }

    impl<'a, T> Iterator for VecIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            let item = self.vec.get(self.index)?;
            self.index += 1;

            Some(item)
        }
    }
}
