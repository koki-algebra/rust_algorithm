use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        queries: [(usize, usize, usize); q],
    }

    let mut idx = vec![Vec::<usize>::new(); n + 1];
    for i in 0..n {
        idx[a[i]].push(i);
    }

    for &(l, r, x) in queries.iter() {
        println!("{}", lower_bound(&idx[x], r) - lower_bound(&idx[x], l - 1));
    }
}

fn lower_bound<T: Ord>(a: &[T], x: T) -> usize {
    let mut left = -1;
    let mut right = a.len() as isize;
    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if a[mid as usize] >= x {
            right = mid;
        } else {
            left = mid;
        }
    }

    right as usize
}
