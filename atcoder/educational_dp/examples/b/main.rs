use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        h: [isize; n],
    }

    let mut dp = vec![usize::MAX; n];
    dp[0] = 0;

    for i in 1..n {
        for j in 1..=k {
            if i >= j {
                dp[i] = dp[i].min(dp[i - j] + h[i].abs_diff(h[i - j]));
            }
        }
    }

    println!("{}", dp[n - 1]);
}
