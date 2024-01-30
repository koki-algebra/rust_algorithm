use std::cmp::min;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        mut a: [i32; n],
        q: i32,
        b: [i32; q],
    }

    a.sort();

    for v in &b {
        // binary search
        let index = match a.binary_search(v) {
            Ok(index) => index,
            Err(index) => index,
        };

        let mut ans = u32::MAX;
        if index != a.len() {
            ans = min(ans, v.abs_diff(a[index]));
        }
        if index >= 1 {
            ans = min(ans, v.abs_diff(a[index - 1]));
        }
        println!("{ans}");
    }
}
