
// Function to do euler numerical approximation
pub fn exp(x:&f64)->f64{
    x.exp()
}

pub fn forward_dif(f: &dyn Fn(f64)->f64, x0:f64, step:f64, end:f64)->[f64]{
    let mut state = x0;
    let num_steps:usize = (((end-x0)%step) + 1.0) as usize;

    let mut steps:Vec<f64> = vec![0.0; num_steps];
    


}