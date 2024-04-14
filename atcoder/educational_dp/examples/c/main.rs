use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        actions: [(usize, usize, usize); n],
    }

    let mut dp = vec![vec![0; 3]; n + 1];

    for i in 1..=n {
        dp[i][0] = actions[i - 1].0 + dp[i - 1][1].max(dp[i - 1][2]);
        dp[i][1] = actions[i - 1].1 + dp[i - 1][0].max(dp[i - 1][2]);
        dp[i][2] = actions[i - 1].2 + dp[i - 1][0].max(dp[i - 1][1]);
    }

    let mut ans = 0;
    for i in 0..3 {
        ans = ans.max(dp[n][i]);
    }

    println!("{}", ans);
}
