use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut users = HashMap::new();
    for (i, user) in s.iter().enumerate() {
        if !users.contains_key(user) {
            println!("{}", i + 1);
            users.insert(user, ());
        }
    }
}
