// this module uses module1

/* use 'as' to rename the function */
// use module1::cool_module::cooler_utility_function as ffs;
// use module1::cool_utility_function;
// use module1::my_add;

/* use {} to import multiple functions */
use module1::{cool_utility_function, inline_module::cooler_utility_function as ffs, my_add, Foo};

fn main() {
    println!("{}", my_add(3, 4));
    cool_utility_function();
    ffs();

    let mut x: Foo = Foo::new(1);
    x.set_x(x.get_x() + 1);
    println!("{}", x.get_x());
}
