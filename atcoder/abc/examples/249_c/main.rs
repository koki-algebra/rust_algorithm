use std::collections::HashMap;

use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: [Chars; n],
    }

    let mut ans = 0;

    for bit in 0..1 << n {
        let mut memo = HashMap::new();
        for i in 0..n {
            if bit >> i & 1 == 1 {
                for &c in s[i].iter() {
                    *memo.entry(c).or_insert(0) += 1;
                }
            }
        }

        let mut cnt = 0;
        for &v in memo.values() {
            if v == k {
                cnt += 1;
            }
        }

        ans = ans.max(cnt);
    }

    println!("{}", ans);
}
