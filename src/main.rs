mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointer_ref;
mod structs;
mod enums;
mod cli;

// fn main() {
//     println!("Hello, Tensor! I'm Rust!");
    // println!("Number of cores: {}", 8);
    // println!("{} {}", "Hello", "Rust");
    // println!("{0} {1}", "Hello", "Rust");
    // println!("{name} {name}", name = "Rust");
    // println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
    // println!("{:?}", (12, true, "Rust"));
    // println!("{number:>width$}", number = 10, width = 6);
    // println!("{number:>0width$}", number = 10, width = 6);
// }

fn main() {
    print::run();
    vars::run();
    types::run();
    strings::run();
    tuples::run();
    arrays::run();
    vectors::run();
    conditionals::run();
    loops::run();
    functions::run();
    pointer_ref::run();
    structs::run();
    enums::run();
    cli::run();
}

// fn main () {
//     let _i = 1 + 2;
//     let _s = 30 - 20;
//     let _m = 10 * 20;
//     let _d = 10 / 5;
//     let _r = 10 % 3;

//     // bool: true or false
//     let _c: char = 'a';

//     // tuple
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     let (_x, y, _z) = tup;
//     println!("The value of y is: {}", y);

//     let a = [1, 2, 3, 4, 5];
//     let _first = a[0];
//     let _second = a[1];
//     let _third = a[2];
//     let _fourth = a[3];
//     let _fifth = a[4];
// }