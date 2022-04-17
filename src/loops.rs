//  Loops - while, for, loop

pub fn run() {
    // While loop
    let mut x = 1;
    while x < 1000 {
        println!("{}", x);
        x += 1;
    }

    // For loop
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}