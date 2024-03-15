use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [isize; n],
        q: usize,
        queries: [(isize, isize); q],
    }

    // sleeps[i]: a[i] 分までに何分寝たか
    let mut sleeps = vec![0; n];
    for i in 1..n {
        if i % 2 == 0 {
            sleeps[i] = sleeps[i - 1] + a[i] - a[i - 1]
        } else {
            sleeps[i] = sleeps[i - 1]
        }
    }

    // x 分までに何分寝たか
    let f = |x| {
        let j = upper_bound(x, &a);
        if j == 0 {
            return 0;
        } else if j == n {
            return sleeps[n - 1];
        }

        return sleeps[j - 1] + (sleeps[j] - sleeps[j - 1]) / (a[j] - a[j - 1]) * (x - a[j - 1]);
    };

    for &(l, r) in queries.iter() {
        println!("{}", f(r) - f(l));
    }
}

fn upper_bound<T: Ord>(item: T, arr: &[T]) -> usize {
    let mut is_asc = true;
    if arr.len() > 1 {
        is_asc = arr[0] < arr[arr.len() - 1];
    }

    let mut left: isize = -1;
    let mut right: isize = arr.len() as isize;
    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if is_asc {
            if arr[mid as usize] > item {
                right = mid;
            } else {
                left = mid;
            }
        } else {
            if arr[mid as usize] > item {
                left = mid;
            } else {
                right = mid;
            }
        }
    }

    right as usize
}
