use proconio::input;

fn main() {
    input! {
        n: isize,
        m: usize,
        d: isize,
        mut a: [isize; n],
        mut b: [isize; m],
    }

    // sort in descending order
    a.sort_by_key(|x| -x);

    let mut ans = -1;
    for &v in b.iter() {
        // binary search
        let mut left = -1;
        let mut right = n;
        while right - left > 1 {
            let mid = left + (right - left) / 2;
            if a[mid as usize] <= v + d {
                right = mid;
            } else {
                left = mid;
            }
        }
        let index = right as usize;

        if index != n as usize && v - a[index] <= d {
            ans = ans.max(a[index] + v);
        }
    }

    println!("{}", ans);
}
