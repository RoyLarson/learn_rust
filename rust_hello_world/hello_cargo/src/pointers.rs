// Pointers are things that reference memory

pub fn run(){
    println!("\nIn pointers.rs");

    // primative array
    let arr1=[1,2,3,4];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    let vec1: Vec<i32> = vec![5,6,7];
    let vec2 = vec1;

    println!("vec2 values: {:?}", vec2);
    // println!("Vec values: {:?}", (vec1, vec2)); //fails
    // error[E0382]: use of moved value: `vec1`
    //     --> src\pointers.rs:15:35
    //     |
    // 12 |     let vec1: Vec<i32> = vec![5,6,7];
    //     |         ---- move occurs because `vec1` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
    // 13 |     let vec2 = vec1;
    //     |                ---- value moved here
    // 14 |
    // 15 |     println!("Vec values: {:?}", (vec1, vec2));
    //     |                                   ^^^^ value used here after move

    // With non-primatives, if you assing another variable to a piece of data, the
    // first variable will no longer hold that value/ref
    // You'll need to use a reference (&) to point to the
    // resource

    let vec1: Vec<i32> = vec![5,6,7];
    let vec2 = &vec1;

    println!("Borrowed from vec1: {:?}", vec2);
    //println!("Vec values: {:?}", (vec1, vec2));  // fails
    //     error[E0505]: cannot move out of `vec1` because it is borrowed
    //     --> src\pointers.rs:35:35
    //      |
    //   34 |     let vec2 = &vec1;
    //      |                ----- borrow of `vec1` occurs here
    //   35 |     println!("Vec values: {:?}", (vec1, vec2));
    //      |                                   ^^^^  ---- borrow later used here
    //      |                                   |
    //      |                                   move out of `vec1` occurs here    

    println!("Borrow vec1 back to get both vec1 and vec2: {:?}", (&vec1, vec2))


}