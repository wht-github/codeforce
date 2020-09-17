use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();
    let testcases: usize = lines.next().unwrap().parse().unwrap();
    for _i in 0..testcases {
        let capicities: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let cnt: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let weights: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let s: usize;
        let w: usize;
        let mut p = capicities[0].min(capicities[1]);
        let f = capicities[0].max(capicities[1]);
        if weights[0] >= weights[1] {
            s = 1;
            w = 0;
        } else {
            s = 0;
            w = 1;
        }
        let Ps = match p / weights[s] >= cnt[s] {
            true => {
                p -= cnt[s] * weights[s];
                cnt[s]
            }
            false => {
                p -= p % weights[s];
                p / weights[s]
            }
        };
    }
    Ok(())
}
