use std::clone;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::{self, Read};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
// struct VecWrapper(Vec<u64>);

#[derive(Clone)]
struct Node(u64, u64, u64); // u,v,w
impl Node {
    pub fn new(arr: Vec<u64>) -> Node {
        Node(arr[0], arr[1], arr[2])
    }
}
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();
    let test_cases: usize = lines.next().unwrap().parse()?;
    for _i in 0..test_cases {
        let n: u64 = lines.next().unwrap().parse()?;
        let arr: Vec<u64> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse::<u64>().unwrap()).collect();
        if arr[0] + arr[1] <= *arr.last().unwrap() {
            println!("1 2 {}", n);
        } else {
            println!("-1");
        }
    }
    Ok(())
}
