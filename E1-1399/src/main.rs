use std::clone;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::{self, Read};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
// struct VecWrapper(Vec<i32>);

#[derive(Clone)]
struct Node(i32, i32, i32); // u,v,w
impl Node {
    pub fn new(arr: Vec<i32>) -> Node {
        Node(arr[0], arr[1], arr[2])
    }
}
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();
    let test_cases: usize = lines.next().unwrap().parse()?;
    for _i in 0..test_cases {
        let mut iter = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap());
        let n = iter.next().unwrap();
        let S = iter.next().unwrap();
        let mut arr = vec![vec![]; n];
        for j in 1..n {
            let node = Node::new(
                lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|v| v.parse::<i32>().unwrap())
                    .collect(),
            );
            arr[node.0 as usize].push((node.1, node.2));
            arr[node.1 as usize].push((node.0, node.2));
        }
        let mut cnt: Vec<i32> = vec![0; n];
        fn dfs(v: usize) {
            
        }
    }
    Ok(())
}

// impl fmt::Display for VecWrapper {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // write!(f, "Values:\n")?;
//         for v in &self.0[0..self.0.len() - 1] {
//             write!(f, "{} ", v)?;
//         }
//         write!(f, "{}", &self.0.last().unwrap())?;
//         Ok(())
//     }
// }
