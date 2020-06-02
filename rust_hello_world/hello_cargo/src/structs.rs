// Working with structs - used to create custom data types
// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
struct TColor(u8, u8, u8);

// Complex Struct
struct Person {
    first_name: String,
    last_name: String
}

impl Person{
    //construct person
    fn new(first: &str, last: &str)-> Person{
        Person{
            first_name:first.to_string(),
            last_name: last.to_string()

        }   
    }
    // Get full name
    fn full_name(&self)->String {
        format!("{} {}", self.first_name, self.last_name)
    }

    //Set last name
    fn set_last_name(&mut self, last:&str){
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self)->(String, String){
        (self.first_name, self.last_name)
    }
}

pub fn run(){
    println!("\nIn structs.rs");
    let mut c= Color{
        red: 255,
        green: 0,
        blue: 0,
    };

    println!("Color: r={} g={} b={}", c.red, c.green, c.blue);

    // change the values of the struct
    c.red = 200;
    println!("Color: r={} g={} b={}", c.red, c.green, c.blue);

    // create a tuple struct
    let mut d = TColor(255, 255, 0);
    println!("TColor: r={}, g={}, b={}", d.0, d.1, d.2);

    d.2 = 255;
    println!("TColor: r={}, g={}, b={}", d.0, d.1, d.2);

    let mut p = Person::new("Me", "Myself");
    println!("Hello: {} {}", p.first_name, p.last_name);

    println!("What is your full name: {}", p.full_name());

    p.set_last_name("I");
    println!("What is your full name now: {}", p.full_name());

    println!("Please record your first and last name: {:?}", p.to_tuple());

    //println!("what does this do: {:?}", p); fails
    // |
    // 74 |     println!("what does this do: {:?}", p)
    //    |                                         ^ `structs::Person` cannot be formatted using `{:?}`
    //    |
    //    = help: the trait `std::fmt::Debug` is not implemented for `structs::Person` 
}