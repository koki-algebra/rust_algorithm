use std::cmp::max;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        points: [(i64, i64); n],
        queries: [usize; q],
    }

    // 45 degree rotation
    let rotated_points: Vec<(i64, i64)> = points.iter().map(|&(x, y)| (x - y, x + y)).collect();

    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    for &(x, y) in &rotated_points {
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    for &i in &queries {
        let (x, y) = rotated_points[i - 1];
        let ans = max(
            max((x - min_x).abs(), (x - max_x).abs()),
            max((y - min_y).abs(), (y - max_y).abs()),
        );

        println!("{}", ans);
    }
}
