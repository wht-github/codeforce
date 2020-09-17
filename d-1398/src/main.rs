use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();
    let rgb: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut arr: Vec<Vec<u32>> = Vec::new();
    for i in 0..3 {
        arr.push(
            lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect(),
        );
        arr[i].sort_by(|a, b| b.cmp(a));
    }
    let mut dp = vec![vec![vec![0; rgb[2] + 1]; rgb[1] + 1]; rgb[0] + 1];
    let mut ans = 0;
    for i in 0..=rgb[0] {
        for j in 0..=rgb[1] {
            for k in 0..=rgb[2] {
                if i < rgb[0] && j < rgb[1] {
                    dp[i + 1][j + 1][k] =
                        dp[i + 1][j + 1][k].max(dp[i][j][k] + arr[0][i] * arr[1][j]);
                }
                if i < rgb[0] && k < rgb[2] {
                    dp[i + 1][j][k + 1] =
                        dp[i + 1][j][k + 1].max(dp[i][j][k] + arr[0][i] * arr[2][k]);
                }
                if j < rgb[1] && k < rgb[2] {
                    dp[i][j + 1][k + 1] =
                        dp[i][j + 1][k + 1].max(dp[i][j][k] + arr[1][j] * arr[2][k]);
                }
                ans = ans.max(dp[i][j][k]);
            }
        }
    }
    println!("{}", ans);
    Ok(())
}
