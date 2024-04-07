use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        queries: [(usize, usize, usize); q],
    }

    let mut top = 0;
    for &(t, x, y) in queries.iter() {
        if t == 1 {
            a.swap((x - 1 + top) % n, (y - 1 + top) % n);
        } else if t == 2 {
            if top > 0 {
                top -= 1;
            } else {
                top = n - 1;
            }
        } else {
            println!("{}", a[(x - 1 + top) % n]);
        }
    }
}
