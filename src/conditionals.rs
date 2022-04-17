// Conditionals - if, else if, else, switch

pub fn run(){
    let age = 12;
    let is_of_age = if age >= 21 {
        true
    } else {
        false
    };
    println!("{}", is_of_age);

    // If else if else
    let is_of_age = if age >= 21 {
        true
    } else if age >= 18 {
        true
    } else {
        false
    };
    println!("{}", is_of_age);

    // Switch
    let number = 6;
    let is_even = match number {
        1 | 3 | 5 | 7 | 9 => false,
        2 | 4 | 6 | 8 | 10 => true,
        _ => false,
    };
    println!("{}", is_even);
}