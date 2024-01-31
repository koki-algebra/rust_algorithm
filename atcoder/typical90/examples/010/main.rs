use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        results: [(i32, i32); n],
        q: usize,
        queries: [(usize, usize); q],
    }

    let mut sums1 = vec![0; n + 1];
    let mut sums2 = vec![0; n + 1];

    for (i, &(c, p)) in results.iter().enumerate() {
        if c == 1 {
            sums1[i + 1] = sums1[i] + p;
            sums2[i + 1] = sums2[i];
        } else if c == 2 {
            sums1[i + 1] = sums1[i];
            sums2[i + 1] = sums2[i] + p;
        }
    }

    for &(l, r) in &queries {
        println!("{} {}", sums1[r] - sums1[l - 1], sums2[r] - sums2[l - 1]);
    }
}
