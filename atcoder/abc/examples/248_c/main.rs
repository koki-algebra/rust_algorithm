use proconio::{fastout, input};

const MOD: usize = 998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    // dp[i][j]: 先頭 i 項決めたときに, 総和が j となる場合の数
    // dp[i][j] = dp[i-1][j-1] + dp[i-1][j-2] + dp[i-1][j-m]
    let mut dp = vec![vec![0; k + 1]; n + 1];
    dp[0][0] = 1;

    for i in 1..=n {
        for j in 1..=k {
            for v in 1..=m {
                if j >= v {
                    dp[i][j] += dp[i - 1][j - v];
                    if dp[i][j] >= MOD {
                        dp[i][j] -= MOD;
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for j in 1..=k {
        ans += dp[n][j];
        if ans >= MOD {
            ans -= MOD;
        }
    }

    println!("{}", ans);
}
