// Vectors are growable arrays - not mathematical vectors - Why didn't they call them lists?
// or something more appropriate
use std::mem;

pub fn run(){
    println!("\nIn vectors.rs");

    // [{type};{len}]
    let array_numbers: [i32; 5] = [1,2,3,4,5];
    let numbers: Vec<i32> = vec![1,2,3,4,5];
    // let numbers: [i32;5] =[1,2,3,4]; // fails because the length of array doesn't match the lenght of assigned values
    // let numbers: [i32;5] =[1,2,3,4, 'a']; // fails because the not all data types are consistent in assignment
    println!("Arrays take {} less bytes then vectors", mem::size_of_val(&numbers)-mem::size_of_val(&array_numbers));

    // Get single value
    println!("the 4th element in numbers is: {}", numbers[3]);

    // to change values make array mutable
    let mut numbers: Vec<i32> =vec![1,2,3,4,5];
    println!("numbers before reassignment {:?}", numbers);
    numbers[2] = 10;

    // Get vector length
    println!("Vector Length: {}", numbers.len());

    // Get memory usage arrays are stack allocated unlike tuples that are heap allocated
    // have to borrow {&} because of stuff
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slices of vectors
    let slice: &[i32] = &numbers;
    println!("Whole Slice: {:?}", slice);

    let slice: &[i32] = &numbers[1..3];
    println!("partial slice [1..3] -> [1..3): {:?}", slice);

    // Make the vector longer
    println!("Adding number individually to vector using .push");
    numbers.push(6);
    numbers.push(10);
    println!("new len of numbers: {}", numbers.len());

    numbers.pop();
    println!("new len of numbers: {}", numbers.len());

    // loop through vector values
    for x in numbers.iter(){
        println!("Number in numbers: {}", x)
    }

    println!("Mutating numbers");
    // loop and mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    };

    println!("numbers at the end of the function: {:?}", numbers);
}