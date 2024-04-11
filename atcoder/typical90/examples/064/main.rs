use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [isize; n],
        queries: [(usize, usize, isize); q],
    }

    let mut b: Vec<isize> = a
        .iter()
        .zip(a.iter().skip(1))
        .map(|(&x, &y)| y - x)
        .collect();
    let mut ans: isize = b.iter().map(|&x| x.abs()).sum();

    for &(mut l, mut r, v) in queries.iter() {
        l -= 1;
        r -= 1;

        let mut before = 0;
        let mut after = 0;

        if l > 0 {
            before += b[l - 1].abs();
            b[l - 1] += v;
            after += b[l - 1].abs();
        }

        if r < n - 1 {
            before += b[r].abs();
            b[r] -= v;
            after += b[r].abs();
        }

        ans += after - before;

        println!("{}", ans);
    }
}
