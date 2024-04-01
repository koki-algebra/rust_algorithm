use std::{collections::HashSet, vec};

use proconio::{fastout, input};

const MAX: usize = 1 << 60;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        m: usize,
        pairs: [(usize, usize); m],
    }

    let mut dangerous = vec![HashSet::new(); n];
    for &(x, y) in pairs.iter() {
        dangerous[x - 1].insert(y - 1);
        dangerous[y - 1].insert(x - 1);
    }

    let mut p: Vec<usize> = (0..n).collect();

    let mut ans: usize = MAX;
    while {
        if is_valid(&p, &dangerous) {
            let mut val = 0;
            for (i, &v) in p.iter().enumerate() {
                val += a[v][i];
            }

            ans = ans.min(val);
        }

        next_permutation(&mut p)
    } {}

    if ans == MAX {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
}

fn next_permutation<T: Ord>(p: &mut [T]) -> bool {
    let n = p.len();
    if n < 2 {
        return false;
    }

    let mut j = n - 1;
    while j > 0 && p[j - 1] >= p[j] {
        j -= 1;
    }

    if j == 0 {
        p.reverse();
        return false;
    }

    let mut l = n - 1;
    while p[l] <= p[j - 1] {
        l -= 1;
    }

    p.swap(j - 1, l);
    p[j..].reverse();

    true
}

fn is_valid(p: &[usize], dangerous: &[HashSet<usize>]) -> bool {
    for i in 0..p.len() - 1 {
        if dangerous[p[i]].contains(&p[i + 1]) {
            return false;
        }
    }
    true
}
