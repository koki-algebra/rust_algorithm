use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        mut coins: [(usize, usize); n],
    }

    let mut a = vec![0; n];
    let mut b = vec![0; n];
    for (i, &(u, v)) in coins.iter().enumerate() {
        a[i] = u;
        b[i] = v;
    }

    let mut dp = vec![vec![false; x + 1]; n + 1];
    dp[0][0] = true;

    for i in 0..n {
        for j in 0..=x {
            for k in 0..=b[i] {
                if j >= a[i] * k {
                    if dp[i][j - a[i] * k] {
                        dp[i + 1][j] = true;
                    }
                }
            }
        }
    }

    if dp[n][x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
