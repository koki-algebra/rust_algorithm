use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: isize,
        k: isize,
        a: [isize; n],
    }

    // 全てのピースの長さを x 以上にできるか？
    let check = |x: isize| -> bool {
        // 前回の切れ目
        let mut prev = 0;
        // ピースの数
        let mut cnt = 0;

        for i in 0..n {
            if a[i] - prev >= x {
                prev = a[i];
                cnt += 1;
            }
        }

        if l - prev >= x {
            cnt += 1;
        }

        return cnt >= k + 1;
    };

    // binary search
    let mut left = -1;
    let mut right = l;
    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if check(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
