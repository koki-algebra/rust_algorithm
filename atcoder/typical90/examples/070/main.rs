use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        points: [(isize, isize); n],
    }

    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for &(x, y) in points.iter() {
        xs.push(x);
        ys.push(y);
    }

    xs.sort();
    ys.sort();

    let a = xs[n / 2];
    let b = ys[n / 2];

    let mut ans_x = 0;
    let mut ans_y = 0;

    for &(x, y) in points.iter() {
        ans_x += x.abs_diff(a);
        ans_y += y.abs_diff(b);
    }

    println!("{}", ans_x + ans_y);
}
