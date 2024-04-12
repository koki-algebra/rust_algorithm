use std::collections::HashMap;

use proconio::{fastout, input};

const MOD: usize = 100000;

#[fastout]
fn main() {
    input! {
        mut n: usize,
        k: usize,
    }

    let mut nums = Vec::new();
    let mut seen = HashMap::new();
    let mut cnt = 0;
    while !seen.contains_key(&n) {
        nums.push(n);
        seen.insert(n, cnt);
        n = (n + digit_sum(n)) % MOD;
        cnt += 1;
    }

    // find: Index of the beginning of the cycle
    if let Some(&find) = seen.get(&n) {
        if k < find {
            println!("{}", nums[k]);
            return;
        }
        let cycle = &nums[find..];
        println!("{}", cycle[(k - find) % cycle.len()]);
    }
}

fn digit_sum(x: usize) -> usize {
    let mut rem = x;
    let mut sum = 0;
    while rem != 0 {
        sum += rem % 10;
        rem /= 10;
    }

    sum
}
