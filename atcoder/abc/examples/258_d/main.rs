use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        stages: [(usize, usize); n],
    }

    let sums: Vec<usize> = stages
        .iter()
        .scan(0, |acc, &stage| {
            *acc += stage.0 + stage.1;
            Some(*acc)
        })
        .collect();

    let mut ans = usize::MAX;
    for m in 0..n {
        if m <= x {
            ans = ans.min(sums[m] + stages[m].1 * (x - m - 1));
        }
    }

    println!("{}", ans);
}
