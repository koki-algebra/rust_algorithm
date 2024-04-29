use std::collections::BTreeMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut s = BTreeMap::new();

    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                x: usize,
            }
            *s.entry(x).or_insert(0) += 1;
        } else if t == 2 {
            input! {
                x: usize,
                c: usize,
            }
            if let Some(&cnt) = s.get(&x) {
                if c >= cnt {
                    s.remove(&x);
                } else {
                    *s.entry(x).or_insert(0) -= c;
                }
            }
        } else {
            let min_key = s.keys().next().unwrap();
            let max_key = s.keys().next_back().unwrap();
            println!("{}", max_key - min_key);
        }
    }
}
