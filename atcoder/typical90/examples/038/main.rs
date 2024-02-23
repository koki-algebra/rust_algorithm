use proconio::{fastout, input};

const INF: usize = 1e18 as usize;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let g = gcd(a, b);
    let tmp = a / g;
    let ans = tmp * b;

    if tmp > INF / b {
        println!("Large");
    } else {
        println!("{}", ans);
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}
