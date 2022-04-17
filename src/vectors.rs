// Vector - Resizable array

pub fn run(){
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    numbers[2] = 20;
    println!("{:?}", numbers);

    // Add on to vector
    numbers.push(5);
    numbers.push(6);
    numbers.push(7);
    println!("{:?}", numbers);

    // Pop off last value
    numbers.pop();
    println!("{:?}", numbers);
    
    // Loop through vector values
    for x in numbers.iter() {
        println!("{}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("{:?}", numbers);
}