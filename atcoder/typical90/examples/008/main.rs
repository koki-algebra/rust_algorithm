use proconio::input;

const MOD: i32 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let t: Vec<char> = String::from("atcoder").chars().collect();
    let m = t.len();

    let mut dp = vec![vec![0; m + 1]; n + 1];
    dp[0][0] = 1;

    for (i, si) in s.chars().enumerate() {
        for j in 0..=m {
            dp[i + 1][j] = (dp[i + 1][j] + dp[i][j]) % MOD;
            if j < m && si == t[j] {
                dp[i + 1][j + 1] = (dp[i + 1][j + 1] + dp[i][j]) % MOD;
            }
        }
    }

    println!("{}", dp[n][m]);
}
