use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    if a < pow(c, b) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn pow(base: usize, exp: usize) -> usize {
    let mut ret = 1;
    for _ in 0..exp {
        ret *= base;
    }

    ret
}
