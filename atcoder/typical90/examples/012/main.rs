use std::mem::swap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }

    let mut uf = UnionFind::new(h * w);
    let mut is_reds = vec![false; h * w];

    for _i in 0..q {
        input! {
            t: usize
        }
        if t == 1 {
            input! {
                mut r: usize,
                mut c: usize,
            }
            r -= 1;
            c -= 1;

            is_reds[index(w, r, c)] = true;

            // up
            if r != 0 && is_reds[index(w, r - 1, c)] {
                uf.union(index(w, r - 1, c), index(w, r, c));
            }
            // down
            if r != h - 1 && is_reds[index(w, r + 1, c)] {
                uf.union(index(w, r + 1, c), index(w, r, c));
            }
            // left
            if c != 0 && is_reds[index(w, r, c - 1)] {
                uf.union(index(w, r, c - 1), index(w, r, c));
            }
            // right
            if c != w - 1 && is_reds[index(w, r, c + 1)] {
                uf.union(index(w, r, c + 1), index(w, r, c));
            }
        } else if t == 2 {
            input! {
                mut ra: usize,
                mut ca: usize,
                mut rb: usize,
                mut cb: usize,
            }
            ra -= 1;
            ca -= 1;
            rb -= 1;
            cb -= 1;

            if is_reds[index(w, ra, ca)]
                && is_reds[index(w, rb, cb)]
                && uf.same(index(w, ra, ca), index(w, rb, cb))
            {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

fn index(w: usize, r: usize, c: usize) -> usize {
    return w * r + c;
}

struct UnionFind {
    parents: Vec<i32>,
    ranks: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parents: vec![-1; n],
            ranks: vec![1; n],
        }
    }

    fn union(&mut self, mut x: usize, mut y: usize) {
        x = self.find(x);
        y = self.find(y);
        if x == y {
            return;
        }
        if self.ranks[x] > self.ranks[y] {
            swap(&mut x, &mut y);
        }
        if self.ranks[x] == self.ranks[y] {
            self.ranks[y] += 1;
        }
        self.parents[x] = y as i32;
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parents[x] < 0 {
            return x;
        }
        self.parents[x] = self.find(self.parents[x] as usize) as i32;

        return self.parents[x] as usize;
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        return self.find(x) == self.find(y);
    }
}
