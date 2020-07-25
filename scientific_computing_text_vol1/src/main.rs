mod euler;

fn main() {
    let x0:f64 =-1.0;
    let beginning:f64= 0.0;
    let end:f64=5.0;
    let step:f64 = 0.01;
    let result = euler::forward_dif(&euler::exp, x0, beginning, end, step);

    println!("{:?}", result);
}
