use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        intervals: [(usize, usize); n],
    }

    // いもす法 O(N + max(R))
    if let Some(max_r) = intervals.iter().map(|&(_, r)| r).max() {
        let mut a = vec![0; max_r + 1];
        for &(l, r) in intervals.iter() {
            a[l] += 1;
            a[r] -= 1;
        }

        for i in 0..max_r {
            a[i + 1] += a[i];
        }

        let mut prev = a[0];
        for i in 1..=max_r {
            if prev == 0 && a[i] != 0 {
                print!("{} ", i);
            }
            if prev != 0 && a[i] == 0 {
                println!("{}", i);
            }
            prev = a[i];
        }
    }
}
