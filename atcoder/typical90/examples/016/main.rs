use std::cmp::min;

use proconio::{fastout, input};

const M: i64 = 9999;

#[fastout]
fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        c: i64,
    }

    let mut ans = 1 << 60;
    for i in 0..=M {
        for j in 0..=M {
            let tmp = n - a * i - b * j;
            if tmp < 0 || tmp % c != 0 {
                continue;
            }
            let k = tmp / c;
            ans = min(ans, i + j + k);
        }
    }

    println!("{ans}");
}
