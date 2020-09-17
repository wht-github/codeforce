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
    for _i in 0..test_cases {
        let _n: usize = lines.next().unwrap().parse()?;
        // let seq: Vec<char> = ;
        let mut pos = HashMap::<char, Vec<i32>>::new();
        pos.insert('0', vec![]);
        pos.insert('1', vec![]);
        let mut t: i32 = 0;
        let mut teams: i32 = 0;
        let mut res = VecWrapper(vec![]);
        // res.push(value: T)
        let mut state: Option<char> = None;
        for c in lines.next().unwrap().chars() {
            match state {
                Some(x) => match x {
                    x if x == c => {
                        t = match pos.get_mut(&get_op(&c)).unwrap().pop() {
                            Some(v) => {
                                pos.get_mut(&c).unwrap().push(t);
                                v
                            }
                            None => {
                                pos.get_mut(&c).unwrap().push(t);
                                teams + 1
                            }
                        };
                        // pos.get_mut(&c).unwrap().push(t);
                        teams = teams.max(t);
                        res.0.push(t);
                        ()
                    }
                    _ => {
                        res.0.push(t);
                        ()
                    }
                },
                None => {
                    t += 1;
                    teams = teams.max(t);
                    res.0.push(t);
                    ()
                }
                
            };
            state = Some(c);
        }
        println!("{}", teams);
        println!("{}", res);
    }
    Ok(())
}
impl fmt::Display for VecWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "Values:\n")?;
        for v in &self.0[0..self.0.len() - 1] {
            write!(f, "{} ", v)?;
        }
        write!(f, "{}", &self.0.last().unwrap())?;
        Ok(())
    }
}
fn get_op(c: &char) -> char {
    match c {
        &'0' => '1',
        &'1' => '0',
        _ => '2',
    }
}
