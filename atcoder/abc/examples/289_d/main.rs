use std::collections::HashSet;

use proconio::input;

const MAX: usize = 100000;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        x: usize,
    }

    let mut traps = HashSet::new();
    for &v in b.iter() {
        traps.insert(v);
    }

    // dp[i]: i 段目に到達可能かどうか
    let mut dp = vec![false; MAX + 1];
    dp[0] = true;
    for i in 0..MAX {
        if !dp[i] {
            continue;
        }
        for &v in a.iter() {
            let next = i + v;
            if next <= MAX {
                dp[next] = !traps.contains(&next);
            }
        }
    }

    if dp[x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
