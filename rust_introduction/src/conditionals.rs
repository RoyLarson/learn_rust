// stuff about conditionals

pub fn run(){
    println!("\nIn conditionals.rs");

    let mut age:u8=18;

    // if else
    if age>=21{
        println!{"Bartender: What would you like to drink?"}
    }else {
        println!{"Bartender: Sorry you have to leave"}
    }
    let mut check_id:bool=false;
    bartender(age, check_id);

    check_id=true;
    bartender(age, check_id);

    age=21;
    check_id=false;
    bartender(age, check_id);

    let mut knows_person:bool=true;
    friendly_bartender(age, check_id, knows_person);

    knows_person=false;
    friendly_bartender(age, check_id, knows_person);

    // short hand if
    let is_of_age = if age>=21 {true} else {false};
    println!("Person is of age: {}", is_of_age)

}

pub fn bartender(age:u8, check_id:bool){
    if age>=21 && check_id{
        println!("Bartender: What would you like to drink?");
    } else if age<21 && check_id{
        println!("Bartender: Sorry you have to leave");
    } else {
        println!("Bartender: I'll need to see an id");      
    }
}

pub fn friendly_bartender(age:u8, check_id:bool, knows_person:bool){
    if age>=21 && check_id||knows_person{
        println!("Bartender: What would you like to drink?");
    } else if age<21 && check_id{
        println!("Bartender: Sorry you have to leave");
    } else {
        println!("Bartender: I'll need to see an id");      
    }
}