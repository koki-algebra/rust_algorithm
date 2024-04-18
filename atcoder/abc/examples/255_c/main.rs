use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: isize,
        a: isize,
        d: isize,
        n: isize,
    }

    if d == 0 || n == 1 {
        println!("{}", a.abs_diff(x));
        return;
    }

    let is_asc = d > 0;

    let mut left = -1;
    let mut right = n;
    while right - left > 1 {
        let mid = left + (right - left) / 2;
        let mid_v = a + mid * d;
        if is_asc {
            if mid_v >= x {
                right = mid;
            } else {
                left = mid;
            }
        } else {
            if mid_v >= x {
                left = mid;
            } else {
                right = mid;
            }
        }
    }

    let mut ans = usize::MAX;
    for i in 0.max(right - 5)..n.min(right + 5) {
        ans = ans.min(x.abs_diff(a + d * i));
    }

    println!("{}", ans);
}
