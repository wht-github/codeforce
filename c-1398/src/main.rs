use rand::Rng;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut rng = rand::thread_rng();
    let mut x0: f64 = 10000000.0;
    let mut x1: f64 = 0.0;
    let mut x2: f64 = 0.0;
    let b: i64 = 10000000;
    for i in 0..b {
        // println!("{}",rng.gen::<f64>());
        if rng.gen::<f64>() < (2.0 * x0 / (2.0 * x0 + x1)) {
            x0 -= 1.0;
            x1 += 1.0;
        } else {
            x1 -= 1.0;
            x2 += 1.0;
        }
    }
    println!("{} {} {}",x0, x1, x2);
    Ok(())
}
