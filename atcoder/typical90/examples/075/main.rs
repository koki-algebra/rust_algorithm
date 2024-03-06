use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    // the number of prime factors of n
    let mut cnt = 0;
    // remainder
    let mut rem = n;

    let mut p = 2;
    while p * p <= n {
        while rem % p == 0 {
            rem /= p;
            cnt += 1;
        }

        p += 1;
    }

    if rem != 1 {
        cnt += 1;
    }

    let mut x = 0;
    while (x as f64) < (cnt as f64).log2() {
        x += 1;
    }

    println!("{}", x);
}
