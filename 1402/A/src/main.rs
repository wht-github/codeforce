use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::{self, Read};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
struct VecWrapper(Vec<i32>);
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();
    let test_cases: usize = lines.next().unwrap().parse()?;
    let __: i32 = 0;
    for _i in 0..test_cases {}
    Ok(())
}
