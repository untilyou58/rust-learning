//  Tuples group multiple values into one compound value.
// Max size of a tuple is 12 elements.

pub fn run(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);
}