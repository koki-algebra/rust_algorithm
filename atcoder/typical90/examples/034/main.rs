use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    let mut num_cnts = HashMap::new();
    let mut right = 0;

    for left in 0..n {
        while right < n {
            if !num_cnts.contains_key(&a[right]) {
                if num_cnts.len() == k {
                    break;
                }
            }
            *num_cnts.entry(a[right]).or_insert(0) += 1;
            right += 1;
        }

        ans = ans.max(right - left);

        if let Some(&cnt) = num_cnts.get(&a[left]) {
            if cnt == 1 {
                num_cnts.remove(&a[left]);
            } else {
                *num_cnts.entry(a[left]).or_insert(0) -= 1;
            }
        }
    }

    println!("{}", ans);
}
