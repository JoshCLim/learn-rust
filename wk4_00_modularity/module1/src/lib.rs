// the 'mod' keyword is used to import a module
pub mod folder_module;
pub mod one_file_module;

// the 'pub' keyword makes the function public (like export)
pub fn my_add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn cool_utility_function() {
    println!("I'm a cool utility function!");
}

// a private function
fn private_fn() {
    println!("I'm a private function!");
}

// [EXAMPLE: public struct]
pub struct Foo {
    x: i32, // fields are private by default
}

// we can implement methods on structs to expose the behaviour we want
impl Foo {
    pub fn new(x: i32) -> Foo {
        Foo { x }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }
}

// [EXAMPLE: declare a module inline]
pub mod inline_module {
    use crate::my_add;

    // the 'pub' keyword makes the function public (like export)
    pub fn cooler_utility_function() {
        println!("I'm a cooler utility function!");

        my_add(4, 5);
    }

    pub fn call_private_fn() {
        // this is allowed because it's in the same module
        super::private_fn();
    }
}

// modules are purely an abstraction for the programmer!!

// [EXAMPLE: importing modules from files or folders]
// see one_file_module.rs for more examples
pub fn use_one_file_module() {
    // module::public_within_this_module(); // this is not allowed because it's private

    // this is allowed because it's in the parent module
    one_file_module::public_within_parent_module();

    // this is allowed because it's in the same crate
    one_file_module::public_within_crate();
}

// see folder_module/mod.rs for more examples
pub fn use_folder_module() {
    folder_module::a::a();
    folder_module::b::b();
}
