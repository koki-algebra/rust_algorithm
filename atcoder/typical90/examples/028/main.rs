use proconio::{fastout, input};

const MAX: usize = 1000;

#[fastout]
fn main() {
    input! {
        n: usize,
        rectangles: [(usize, usize, usize, usize); n],
    }

    let mut sums: Vec<Vec<i32>> = vec![vec![0; MAX + 1]; MAX + 1];

    for (lx, ly, rx, ry) in rectangles {
        sums[lx][ly] += 1;
        sums[lx][ry] -= 1;
        sums[rx][ry] += 1;
        sums[rx][ly] -= 1;
    }

    for i in 0..=MAX {
        for j in 0..MAX {
            sums[i][j + 1] += sums[i][j];
        }
    }
    for j in 0..=MAX {
        for i in 0..MAX {
            sums[i + 1][j] += sums[i][j];
        }
    }

    let mut ans = vec![0; n + 1];
    for i in 0..=MAX {
        for j in 0..=MAX {
            ans[sums[i][j] as usize] += 1;
        }
    }

    for i in 0..n {
        println!("{}", ans[i + 1]);
    }
}
