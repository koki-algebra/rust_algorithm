use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        queries: [(usize, usize); q],
    }

    let mut p = 0;
    for &(t, x) in queries.iter() {
        if t == 1 {
            p = (p + x) % n;
        } else {
            println!("{}", s[(n + x - 1 - p) % n]);
        }
    }
}
