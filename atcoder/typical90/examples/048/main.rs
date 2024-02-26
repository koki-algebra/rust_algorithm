use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        points: [(usize, usize); n],
    }

    let mut v = Vec::new();
    for &(a, b) in points.iter() {
        v.push(b);
        v.push(a - b);
    }

    v.sort_by(|a, b| b.cmp(a));

    let mut ans = 0;
    for t in 0..k {
        ans += v[t];
    }

    println!("{}", ans);
}
