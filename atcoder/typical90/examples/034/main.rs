use std::{cmp::max, collections::HashMap};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut right = 0;
    let mut kind = 0;
    let mut num_cnts: HashMap<usize, usize> = HashMap::new();
    let mut ans = 0;

    for left in 0..n {
        while right < n {
            if !num_cnts.contains_key(&a[right]) || num_cnts.get(&a[right]) == Some(&0) {
                if kind == k {
                    break;
                }
                kind += 1;
            }
            *num_cnts.entry(a[right]).or_insert(0) += 1;
            right += 1;
        }

        ans = max(ans, right - left);
        if num_cnts.get(&a[left]) == Some(&1) {
            kind -= 1;
        }
        *num_cnts.entry(a[left]).or_insert(0) -= 1;
    }

    println!("{}", ans);
}
