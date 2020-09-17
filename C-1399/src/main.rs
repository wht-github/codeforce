use std::error::Error;
use std::io::{self, Read};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();
    let test_cases: i32 = lines.next().unwrap().parse()?;
    for i in 0..test_cases {
        let mut ans: i32 = 0;
        let n: i32 = lines.next().unwrap().parse()?;
        let mut cnt: Vec<i32> = vec![0; (n + 1) as usize];
        for x in lines.next().unwrap().split_whitespace() {
            cnt[x.parse::<usize>().unwrap()] += 1;
        }
        for s in 2..=2 * n {
            let mut cur: i32 = 0;
            for i in 1..(s + 1) / 2 {
                if s - i <= n {
                    cur += cnt[i as usize].min(cnt[(s - i) as usize]);
                }
            }
            if s % 2 == 0 {
                cur += cnt[(s / 2) as usize] / 2;
            }
            ans = ans.max(cur);
        }
        println!("{:?}", ans);
    }
    Ok(())
}
