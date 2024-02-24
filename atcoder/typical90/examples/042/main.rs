use std::cmp::min;

use proconio::{fastout, input};

const MOD: usize = 1000000007;

#[fastout]
fn main() {
    input! {
        k: usize,
    }

    if k % 9 != 0 {
        println!("{}", 0);
        return;
    }

    let mut dp = vec![0; k + 1];
    dp[0] = 1;

    for i in 1..=k {
        let b = min(i, 9);
        for j in 1..=b {
            dp[i] += dp[i - j];
            if dp[i] >= MOD {
                dp[i] -= MOD;
            }
        }
    }

    println!("{}", dp[k]);
}
