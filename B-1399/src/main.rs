use std::error::Error;
use std::io::{self, Read};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();
    let test_cases: i64 = lines.next().unwrap().parse()?;
    for i in 0..test_cases {
        let n: i64 = lines.next().unwrap().parse()?;
        let a: Vec<i64> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let b: Vec<i64> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let a_min = a.iter().min().unwrap();
        let b_min = b.iter().min().unwrap();
        let gifts: Vec<_> = a.iter().zip(b.iter()).collect();
        let sum: i64 = gifts.iter().map(|(&a,&b)| {(a-a_min).max(b-b_min)}).sum();
        println!("{}",sum);
    }
    Ok(())
}
