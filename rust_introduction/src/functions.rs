// Working with functions


pub fn run(){
    println!("\nIn functions.rs");
    greeting("Hello", "I");
    let a = 3;
    let b = 4;

    // Bind variables to result of function
    let x = add(a, b);
    println!("adding {}+{}={}", a, b, x);

    // Closure: lambda in python
    let add_nums = |n1:i32, n2:i32| n1+n2;
    println!("closure sum: {}", add_nums(10, 20));

    let n3:i32 = 10;
    let add_10 = |n1:i32| n1+n3;
    println!("Add 10: {}", add_10(10));

    let add_each_plus_10 = |n1:i32, n2:i32| n1+n2+n3;
    println!("Add each and 10: {}", add_each_plus_10(3,5))

}

fn greeting(greet:&str, name:&str){
    println!("{} {}, nice to meat you.",greet, name );
}

fn add(n1:i32, n2:i32)->i32{
    // Implicit return when line doesn't have semicolon ;
    n1+n2
}