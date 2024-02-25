use std::collections::VecDeque;

use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: isize,
        w: isize,
        start: (isize, isize),
        goal: (isize, isize),
        s: [Chars; h],
    }

    // start
    let rs = start.0 - 1;
    let cs = start.1 - 1;
    // goal
    let rg = goal.0 - 1;
    let cg = goal.1 - 1;

    let dr = vec![0, 1, 0, -1];
    let dc = vec![1, 0, -1, 0];

    let mut dist = vec![vec![vec![isize::MAX; 4]; w as usize]; h as usize];

    let mut deque = VecDeque::new();
    for dir in 0..4 {
        deque.push_back((rs, cs, dir));
        dist[rs as usize][cs as usize][dir] = 0;
    }

    while let Some((now_r, now_c, now_dir)) = deque.pop_front() {
        for next_dir in 0..4 {
            let next_r = now_r + dr[next_dir];
            let next_c = now_c + dc[next_dir];

            let mut cost = dist[now_r as usize][now_c as usize][now_dir];
            if next_dir != now_dir {
                cost += 1;
            }

            if next_r >= 0
                && next_r < h
                && next_c >= 0
                && next_c < w
                && s[next_r as usize][next_c as usize] == '.'
                && chmin(&mut dist[next_r as usize][next_c as usize][next_dir], cost)
            {
                if next_dir != now_dir {
                    deque.push_back((next_r, next_c, next_dir));
                } else {
                    deque.push_front((next_r, next_c, next_dir));
                }
            }
        }
    }

    let mut ans = isize::MAX;
    for dir in 0..4 {
        chmin(&mut ans, dist[rg as usize][cg as usize][dir]);
    }

    println!("{}", ans);
}

fn chmin(a: &mut isize, b: isize) -> bool {
    if *a > b {
        *a = b;
        true
    } else {
        false
    }
}
