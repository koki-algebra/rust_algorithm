use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut diff = 0;
    for i in 0..n {
        diff += a[i].abs_diff(b[i]);
    }

    if k < diff || diff.abs_diff(k) % 2 != 0 {
        println!("No");
    } else {
        println!("Yes");
    }
}
