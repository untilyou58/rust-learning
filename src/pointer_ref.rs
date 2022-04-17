// Pointer Ref Type - Point to a resource and keep a reference to it.

pub fn run() {
    let arr1 = [1, 2, 3, 4, 5];
    let _arr2 = arr1;

    println!("{:?}", arr1);

    // With non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value.
    // You'll need to use a reference (&) to the data instead. This is called a reference.
    let vec1 = vec![1, 2, 3];
    let _vec2 = &vec1;

    println!("{:?}", vec1);
}