use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut users = HashSet::new();
    for (day, name) in s.iter().enumerate() {
        if !users.contains(name) {
            println!("{}", day + 1);
            users.insert(name.clone());
        }
    }
}
