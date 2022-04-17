//  Funtion - Used to store code snippets that get called when the program is run

pub fn run() {
    greeting("Tensor", "Rust");

    //  Bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closure
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3, 3));
}

fn greeting(name: &str, language: &str) {
    println!("Hello {0}! I'm {1} and I'm learning {0}", name, language);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}