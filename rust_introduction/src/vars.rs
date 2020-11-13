// Variables hold premative data or references to data
// variables are immutable by default
// Rust is a block scoped langauge if you set a variable
// in a function it is scoped only to that function
pub fn run() {
    println!("\nIn vars.rs");
    let name = "Myself"; // I can't change this
    // let idea = "A good idea";
    // idea = "Now I have a bad idea";
    // warning: value assigned to `idea` is never read
    //  --> src\vars.rs:7:9
    //   |
    // 7 |     let idea = "A good idea";
    //   |         ^^^^
    //   |
    //   = note: `#[warn(unused_assignments)]` on by default
    //   = help: maybe it is overwritten before being read?
    // error[E0384]: cannot assign twice to immutable variable `idea`
    //  --> src\vars.rs:9:5
    //   |
    // 7 |     let idea = "A good idea";
    //   |         ----------
    //   |         |
    //   |         first assignment to `idea`
    //   |         help: make this binding mutable: `mut idea`
    // 8 |
    // 9 |     idea = "Now I have a bad idea";
    //   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable 

    let mut idea = "a good idea";
    println!("Me, {}, and I have {}", name, idea);
    idea = "now I have a bad idea";

    println!("Me, {}, and I have {}", name, idea);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // multiple assignment
    let (myself, idea) = ("Myself", "an okay idea");
    println!("Me, {}, and I have {}", myself, idea);

}