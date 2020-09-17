use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let take2 = |v: Vec<_>| (v[0], v[1]);
    for _i in 0..n {
        let (tp, d) = take2(
            lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect(),
        );
        
    }
    Ok(())
}
