use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [isize; n],
        changes: [(usize, usize, isize); q],
    }

    let mut ans = 0;
    let mut b = vec![0; n - 1];
    for i in 0..n - 1 {
        b[i] = a[i + 1] - a[i];
        ans += b[i].abs();
    }

    for &(mut l, mut r, v) in changes.iter() {
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
