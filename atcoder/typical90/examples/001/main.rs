use proconio::input;

fn main() {
    input! {
        n: i32,
        l: i32,
        k: i32,
        a: [i32; n],
    }

    let check = |x: i32| -> bool {
        let mut num = 0;
        let mut prev = 0;
        for &val in a.iter() {
            if val - prev >= x {
                num += 1;
                prev = val;
            }
        }

        if l - prev >= x {
            num += 1;
        }

        num >= k + 1
    };

    // Binary Search
    let mut left = -1;
    let mut right = l + 1;
    while right - left > 1 {
        let mid = (left + right) / 2;
        if check(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{left}");
}
