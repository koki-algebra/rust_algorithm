use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m]
    }

    let mut cnts = vec![0; n];
    for &(a, b) in edges.iter() {
        if a > b {
            cnts[a - 1] += 1;
        }
        if b > a {
            cnts[b - 1] += 1;
        }
    }

    let mut ans = 0;
    for &v in cnts.iter() {
        if v == 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
