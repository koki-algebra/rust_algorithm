use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize
    }

    let mut memo: HashMap<usize, usize> = HashMap::new();

    println!("{}", f(n, &mut memo))
}

fn f(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if n == 0 {
        return 1;
    }

    if let Some(&ret) = memo.get(&n) {
        return ret;
    }

    let ret = f(n / 2, memo) + f(n / 3, memo);
    memo.insert(n, ret);

    ret
}
