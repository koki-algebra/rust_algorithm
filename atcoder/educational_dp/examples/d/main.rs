use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        goods: [(usize, usize); n],
    }

    let mut dp = vec![vec![0; w + 1]; n + 1];
    for (i, &(weight, value)) in goods.iter().enumerate() {
        for j in 0..=w {
            if j >= weight {
                dp[i + 1][j] = dp[i][j].max(dp[i][j - weight] + value);
            } else {
                dp[i + 1][j] = dp[i][j];
            }
        }
    }

    println!("{}", dp[n][w]);
}
