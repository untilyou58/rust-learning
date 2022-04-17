// Pritive str = immutable fixed-length string (contains no null terminator) somewhere in memory
// String = growable, heap-allocated string - Use when you need to store a dynamically sized string

pub fn run() {
    let mut hello = String::from("Hello");

    // Get length
    println!("Length: {}", hello.len());

    hello.push(' ');
    hello.push_str("World!");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Is empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    
}