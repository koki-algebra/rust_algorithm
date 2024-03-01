use std::collections::VecDeque;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
        queries: [(usize, usize); q],
    }

    let mut deque = VecDeque::new();
    for &(t, x) in queries.iter() {
        if t == 1 {
            deque.push_front(x);
        } else if t == 2 {
            deque.push_back(x);
        } else {
            println!("{}", deque[x - 1]);
        }
    }
}
