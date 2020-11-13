// Working with loops

pub fn run(){
    println!("\nIn loops.rs");

    let mut count=0;
    // infinite loop
    loop {
        count +=1;
        println!("count: {}", count);
        if count >= 5{
            println!("Forcing exit with break");
            break;
        }
    }

    count = 0;
    // While loop (fizz buzz)
    println!("While Loop FizzBuzz");
    while count<=20 {
        fizzbuzz(count);
        count +=1;
    }

    // for range
    println!("For x in 0..10 FizzBuss");
    for x in 0..20{
        fizzbuzz(x);
    }


}

pub fn fizzbuzz(count:i32){
    if count%15==0{
        println!("Fizzbuzz");
    } else if count % 3 == 0 {
        println!("Fizz");
    } else if count % 5 == 0 {
         println!("Buzz");
    } else {
        println!("Count: {}", count);
    }

}