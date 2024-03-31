use proconio::{fastout, input};

const MAX: usize = 1000;

#[fastout]
fn main() {
    input! {
        n: usize,
        points: [(usize, usize, usize, usize); n],
    }

    let mut area: [[i32; MAX + 1]; MAX + 1] = [[0; MAX + 1]; MAX + 1];
    for &(lx, ly, rx, ry) in points.iter() {
        area[ly][lx] += 1;
        area[ry][lx] -= 1;
        area[ly][rx] -= 1;
        area[ry][rx] += 1;
    }

    for y in 0..=MAX {
        for x in 0..MAX {
            area[y][x + 1] += area[y][x];
        }
    }

    for x in 0..=MAX {
        for y in 0..MAX {
            area[y + 1][x] += area[y][x];
        }
    }

    let mut ans = vec![0; n + 1];
    for y in 0..=MAX {
        for x in 0..=MAX {
            ans[area[y][x] as usize] += 1;
        }
    }

    for i in 0..n {
        println!("{}", ans[i + 1]);
    }
}
