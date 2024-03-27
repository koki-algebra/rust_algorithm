use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        a: [usize; n],
    }

    let mut sums = HashSet::new();
    sums.insert(0);
    let mut sum = 0;
    for &v in a.iter() {
        sum += v;
        sums.insert(sum);
    }

    for &x in sums.iter() {
        if sums.contains(&(x + p)) && sums.contains(&(x + p + q)) && sums.contains(&(x + p + q + r))
        {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
