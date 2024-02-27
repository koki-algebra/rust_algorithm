use proconio::{fastout, input};

const MOD: usize = 1000000007;

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
    }

    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    for i in 1..=n {
        if i >= l {
            dp[i] = (dp[i - 1] + dp[i - l]) % MOD;
        } else {
            dp[i] = dp[i - 1];
        }
    }

    println!("{}", dp[n]);
}
