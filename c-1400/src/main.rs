use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};
use std::iter::FromIterator;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();
    let testcases: usize = lines.next().unwrap().parse().unwrap();
    for _i in 0..testcases {
        let s: Vec<char> = lines.next().unwrap().trim().chars().collect();
        let x: usize = lines.next().unwrap().parse().unwrap();
        let mut w = vec!['1'; s.len()];
        for j in 1..=s.len() {
            if s[j - 1] == '0' {
                if j > x {
                    w[j - x - 1] = '0';
                }
                if j + x <= s.len() {
                    w[j + x - 1] = '0';
                }
            }
        }
        let mut flag = true;
        for j in 1..=w.len() {
            if s[j - 1] == '1' {
                if (j > x && w[j - x - 1] == '1') || (j + x <= w.len() && w[j + x - 1] == '1') {

                } else {
                    flag = false;
                }
            }
        }
        println!(
            "{}",
            match flag {
                true => String::from_iter(w.into_iter()),
                false => (-1).to_string(),
            }
        );
    }
    Ok(())
}
