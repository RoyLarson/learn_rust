// Tuples group together values, and they don't have to be the same type like and array
// Max 12 elements

pub fn run() {
    println!("\nIn tuples.rs");

    let thing: (&str, i32, &str, i16, &str, i16) = ("Computer", 1600, "Keyboard", 45, "Mouse", 15);

    // Rust may not unpack the tuple itself
    // println!("{} cost {}, {} cost {}, {} cost {}", *thing); // does not work
    println!("{} cost {}, {} cost {}, {} cost {}", thing.0, thing.1, thing.2, thing.3, thing.4, thing.5);


    println!("{:?}", thing);

}