// Workiing with Enums:

enum Movement{
    //Variant
    Up,
    Down,
    Left,
    Right,
}

fn move_avator(m: Movement){
    // Perform action depending on info
    match  m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right"),
    }
}

pub fn run(){
    println!("\nIn enums.rs");

    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avator(avatar1);
    move_avator(avatar2);
    move_avator(avatar3);
    move_avator(avatar4);

}