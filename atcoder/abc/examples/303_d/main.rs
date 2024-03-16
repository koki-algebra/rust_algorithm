use proconio::{fastout, input, marker::Chars};

const INF: usize = 1 << 60;

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
        s: Chars,
    }

    let n = s.len();
    let mut dp = vec![vec![INF; 2]; n + 1];
    dp[0][0] = 0;

    for i in 0..n {
        let mut cur = 0;
        if s[i] == 'A' {
            cur = 1;
        }

        for j in 0..2 {
            for k in 0..2 {
                let mut v = dp[i][j];
                if j != k {
                    v += z;
                }
                if cur == k {
                    v += x;
                } else {
                    v += y;
                }
                chmin(&mut dp[i + 1][k], v);
            }
        }
    }

    println!("{}", dp[n][0].min(dp[n][1]));
}

fn chmin(a: &mut usize, b: usize) -> bool {
    if *a > b {
        *a = b;
        true
    } else {
        false
    }
}
