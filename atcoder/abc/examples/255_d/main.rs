use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [isize; n],
        x: [isize; q],
    }

    a.sort();

    let mut sums = vec![0; n + 1];
    for i in 0..n {
        sums[i + 1] = sums[i] + a[i];
    }

    for i in 0..q {
        let mut left = -1;
        let mut right = n as isize;

        while right - left > 1 {
            let mid = left + (right - left) / 2;
            if x[i] <= a[mid as usize] {
                right = mid;
            } else {
                left = mid;
            }
        }

        let k = right as usize;

        let mut ans = sums[n] - sums[k];
        ans -= sums[k];
        ans += 2 * k as isize * x[i];
        ans -= n as isize * x[i];

        println!("{}", ans);
    }
}
