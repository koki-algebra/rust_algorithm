use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }

    if n == 1 {
        println!("{}", 0);
        return;
    }

    let mut r = vec![0; n + 1];
    let mut b = vec![0; n + 1];
    b[1] = 1;
    for i in 1..n {
        b[i+1] = r[i] + y * b[i];
        r[i+1] = r[i] + x * b[i+1];
    }

    println!("{}", r[n]);
}
