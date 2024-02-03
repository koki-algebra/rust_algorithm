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
    let mut ans = 1;
    for _i in 0..exp {
        ans *= base;
    }

    return ans;
}
