use std::env;

pub fn run(){
    println!("\nIn command_line.rs");

    let args: Vec<String> = env::args().collect();

    println!("Args: {:?}", args);
    //cargo run ->
    //Args: ["target\\debug\\hello_cargo.exe"]
    //cargo run "hello" ->
    //Args: ["target\\debug\\hello_cargo.exe", "hello"]
    //cargo run hello world ->
    //Args: ["target\\debug\\hello_cargo.exe", "hello", "world"]

    let command = args[1].clone();
    let name = args[2].clone();
    println!("Command is: {}", command);
    //cargo run hello ->
    //Command is: hello

    if command=="hello"{
        println!("Hi how are you {}", name)
    }
    //cargo run hello world ->
    //Hi how are you world

    //cargo run hello ->
    // thread 'main' panicked at 'index out of bounds: the len is 2 but the index is 2', /rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447\src\libcore\slice\mod.rs:2791:10
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // error: process didn't exit successfully: `target\debug\hello_cargo.exe hello` (exit code: 101)
}
