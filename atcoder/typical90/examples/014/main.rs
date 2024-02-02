use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    }

    a.sort();
    b.sort();

    let mut ans: usize = 0;
    for i in 0..n {
        ans += a[i].abs_diff(b[i]);
    }

    println!("{ans}");
}
