/*
Primitive types
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/
// Rust is a statically typed language, which means that it must know the types of all variables at compile time,
// however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run() {
    // Default is "i32"
    let _x = 1;

    // Default is "f64"
    let _y = 2.5;

    // Add explicit type
    let _z: i64 = 50000;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let _t = true;
    let _f: bool = false; // with explicit type

    // Get boolean from expression
    let _t2 = !_t;

    // Char
    let _c = 'z';
    let _z2 = 'ℤ';
    let _heart_eyed_cat = '😻';

    // Tuples
    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x2, _y2, _z2) = _tup;

    println!("The value of y is: {}", _y2);

    let a = [1, 2, 3, 4, 5];
    let _first = a[0];
    let _second = a[1];
    let _third = a[2];
    let _fourth = a[3];
    let _fifth = a[4];

    // Array length
    println!("Array length: {}", a.len());

    // Arrays are stack allocated
    // Arrays are fixed size
    // Arrays are accessed by index
    // Arrays are growable
    // Arrays are copy by value
}
