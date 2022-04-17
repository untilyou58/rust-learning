pub fn run() {
    let name = "Rust";
    let mut age = 30;
    println!("My name is {} and I'm {}", name, age);
    age = 31;

    println!("My name is {} and I'm {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Rust", 30);
    println!("My name is {} and I'm {}", my_name, my_age);

    // println!("My name is {}", name);
}