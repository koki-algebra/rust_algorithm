use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [isize; n],
    }

    let mut dp = vec![usize::MAX; n];
    dp[0] = 0;
    for i in 1..n {
        dp[i] = dp[i-1] + h[i].abs_diff(h[i-1]);
        if i > 1 {
            dp[i] = dp[i].min(dp[i-2] + h[i].abs_diff(h[i-2]));
        }
    }

    println!("{}", dp[n - 1]);
}
