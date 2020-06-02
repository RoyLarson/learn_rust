
// Primative strings = Immutable fixed length string somewhere in memory
// String = Growable, heap-allocated  data structure -
//     Use when you need to modify or own string data

pub fn run(){
    println!("\nIn strings.rs"); 
    let prim_hello = "hello";
    let mut str_hello = String::from("hello world");

    // add one char to the string
    str_hello.push(' ');

    // add a string to the string
    str_hello.push_str("live from Mars");
    // get length
    println!("Length of prim_hello: {} length of str_hello: {}", prim_hello.len(), str_hello.len());

    // Capacity in bytes
    println!("str_hello capacity: {}", str_hello.capacity());

    // Check if is empty
    println!("str_hello capacity: {}", str_hello.is_empty());

    // Check if contains
    println!("str_hello contains 'Mars': {}", str_hello.contains("Mars"));

    // Replace parts
    println!("Replace: {}", str_hello.replace("Mars", "Saturn"));

    // Loop through string by whitespace
    for word in str_hello.split_whitespace() {
        println!("A word in str_hello: {}", word);
    };

    // Create a string with a capacity
    let mut set_cap = String::with_capacity(10);
    set_cap.push('a');
    set_cap.push('b');

    // writing assertions
    assert_eq!(2, set_cap.len());
    set_cap.push('c');
    // assert_eq!(2, set_cap.len());  // this will cause a failure

    assert_eq!(10, set_cap.capacity());

    println!("{:?}", (prim_hello, str_hello, set_cap));
    // This will fail because of borrow checker apparently println! owned the data when I called it like that
    // println!("Length of prim_hello: {} length of str_hello: {}", prim_hello.len(), str_hello.len()); 


}