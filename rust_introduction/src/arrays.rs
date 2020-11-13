// Arrays are fixed length specific type data structures

pub fn run(){
    println!("\nIn arrays.rs");

    // [{type};{len}]
    let numbers: [i32;5] =[1,2,3,4,5];
    // let numbers: [i32;5] =[1,2,3,4]; // fails because the length of array doesn't match the length of assigned values
    // let numbers: [i32;5] =[1,2,3,4, 'a']; // fails because the not all data types are consistent in assignment

    // Get single value
    println!("the 4th element in numbers is: {}", numbers[3]);

    // to change values make array mutable
    let mut numbers: [i32;5] =[1,2,3,4,5];
    println!("numbers before reassignment {:?}", numbers);
    numbers[2] = 10;

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Get memory usage arrays are stack allocated unlike tuples that are heap allocated
    // have to borrow {&} because of stuff
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get slices of array
    let slice: &[i32] = &numbers;
    println!("Whole Slice: {:?}", slice);

    let slice: &[i32] = &numbers[1..3];
    println!("partial slice [1..3] -> [1..3): {:?}", slice);

    println!("numbers at the end of the function: {:?}", numbers);
}