pub fn run() {
    // / Print to console
    println!("\nIn print.rs");

    // basic string formating
    println!("{} is greater than {}", 1, 2);

    // Positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "A", "c", "b");

    // Named arguments
    println!("{name} lists to play {activity}", name="A", activity="C");

    // Place holder traits
    println!("Binary: {:b} Hex: {:x} Oct: {:o}", 10 ,10, 10);

    // Debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10+10);

    // What happens if
    // println!("A + B = {}", "A", "B")
    //     --> src\print.rs:24:31
    //     |
    //  24 |     println!("A + B = {}", "A"+"B");
    //     |                            ---^--- &str
    //     |                            |  |
    //     |                            |  `+` cannot be used to concatenate two `&str` strings
    //     |                            &str
    //     |
    //  help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
    //     |
    //  24 |     println!("A + B = {}", "A".to_owned()+"B");
    //     |                            ^^^^^^^^^^^^^^
    println!("A + B = {}", "A".to_owned()+"B");

}