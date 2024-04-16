use proconio::input;

const INF: usize = 1 << 60;
const MAX_V: usize = 100000;

fn main() {
    input! {
        n: usize,
        w: usize,
        goods: [(usize, usize); n],
    }

    // dp[i][v]: i 番目の品物までを使って, 価値の総和を v にするときの重さの総和の最小値
    let mut dp = vec![[INF; MAX_V + 1]; n + 1];
    dp[0][0] = 0;

    for (i, &(weight, value)) in goods.iter().enumerate() {
        for v in 0..=MAX_V {
            if v >= value {
                dp[i + 1][v] = dp[i + 1][v].min(dp[i][v - value] + weight);
            }
            dp[i + 1][v] = dp[i + 1][v].min(dp[i][v]);
        }
    }

    let mut ans = 0;
    for v in 1..=MAX_V {
        if dp[n][v] <= w {
            ans = v;
        }
    }

    println!("{}", ans);
}
