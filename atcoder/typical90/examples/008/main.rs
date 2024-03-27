use proconio::{fastout, input, marker::Chars};

const MOD: usize = 1000000007;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let t: Vec<char> = "atcoder".chars().collect();

    // S の i 文字目までで, T の j 文字目までを作る方法の数
    let mut dp = vec![vec![0; t.len() + 1]; n + 1];
    dp[0][0] = 1;
    for i in 1..=n {
        for j in 0..=t.len() {
            dp[i][j] = dp[i - 1][j];
            if j > 0 && s[i - 1] == t[j - 1] {
                dp[i][j] += dp[i - 1][j - 1];
                dp[i][j] %= MOD;
            }
        }
    }

    println!("{}", dp[n][t.len()])
}
