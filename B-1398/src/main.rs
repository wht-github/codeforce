use std::error::Error;
use std::io::{self, Read};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();
    let test_cases: usize = lines.next().unwrap().parse()?;
    for _i in 0..test_cases {
        let mut arr:Vec<usize>= lines.next().unwrap().split('0').map(|x| x.len()).filter(|&x| x > 0).collect();
        arr.sort_by(|a, b| b.cmp(a));
        println!("{}", arr.iter().step_by(2).sum::<usize>());
    }
    Ok(())
}
