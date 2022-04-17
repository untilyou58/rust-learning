use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    let command = args[1].clone();
    let name = "Rust";

    if command == "hello" {
        println!("Hello, {}", name);
    } else if command == "num" {
        let num = 10;
        println!("{}", num);
    } else {
        println!("Command not recognized");
    }
}