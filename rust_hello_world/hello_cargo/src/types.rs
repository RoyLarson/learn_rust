/* Primative types in rust
integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 -> {type:(u)nsigned(+ only), (i)nteger(+&-)}{bits allocated}
floats: f32, f64
boolean: (bool)
characters: (char)
tuples: (val0, val1, val2, ...) 
arrays: [val0, val1, val2, ...]? fixed length
*/

/*Rust is statically typed, so it must know the types of all variables at compile time
A lot of the time the compiler can infer the type based on what it is assigned to
and how it is being used
*/

pub fn run() {
    println!("\nIn types.rs");
    // Default for below is i32
    let x = 1;
    // Default is float64
    let y = 2.5;
    // Explicit type
    let z: i64 = 24231231324412;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let active= true;

    // Get boolean from expression
    let is_greater = 10>5;

    // char
    let a1 =  'a';   // Signifies a char unicode char
    let face = '\u{1F600}'; //doesn't print out in windows command prompt well

    println!("{:?}",(x, y, z, active, is_greater, a1, face));


}