
// Function to do euler numerical approximation
pub fn exp(x:&f64)->f64{
    x.exp()
}

pub fn forward_dif(f: &dyn Fn(&f64)->f64, x0:f64, beginning:f64,  end:f64, step:f64)->(Vec<f64>, Vec<f64>){
    let mut state = x0;
    let mut time = beginning;
    let num_steps:usize = (((end-beginning)/step) + 1.0) as usize;
    println!("Num_steps: {}", &num_steps);

    let mut steps:Vec<f64> = vec![0.0; num_steps];
    let mut results:Vec<f64> = vec![0.0; num_steps];

    for (t, x) in steps.iter_mut().zip(results.iter_mut()) {

        println!("T: {} X: {} Time: {} state: {}", t, x, time, state);
        time += step;
        state += f(&(-1.0*time));
        *t = time;
        *x = state;
        // println!("T: {} X: {}", t, x)
        // *x = state;
        // state += f(&state);
    }
    return (steps, results)


}