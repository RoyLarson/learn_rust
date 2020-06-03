mod euler;
use std::f64;


fn main() {
    let x0:f64 =1.0;
    let end:f64=5.0;
    let step:f64 = .01
    euler::forward_dif(euler::exp, x0, step, end)

    println!("Hello, world!");
}
