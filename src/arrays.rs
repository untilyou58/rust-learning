//  Arrays - Fixed list where elements are the same data types.

pub fn run(){
    let a = [1, 2, 3, 4, 5];
    let _first = a[0];
    let _second = a[1];
    let _third = a[2];
    let _fourth = a[3];
    let _fifth = a[4];
    
    println!("{:?}", a);

    // Array are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&a));

    // Get length
    println!("Length: {}", a.len());

    // Check if empty
    println!("Is empty: {}", a.is_empty());

    // Get slice
    let slice = &a[0..2];
    println!("Slice: {:?}", slice);
}