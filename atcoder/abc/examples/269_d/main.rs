use std::{
    collections::{HashMap, HashSet},
    mem::swap,
};

use proconio::input;

fn main() {
    input! {
        n: usize,
        pairs: [(isize, isize); n],
    }

    let mut points = HashMap::new();
    for (i, &(x, y)) in pairs.iter().enumerate() {
        points.insert((x, y), i);
    }

    let mut uf = UnionFind::new(n);

    for (&(x, y), &u) in points.iter() {
        if let Some(&v) = points.get(&(x - 1, y - 1)) {
            uf.union(u, v);
        }
        if let Some(&v) = points.get(&(x - 1, y)) {
            uf.union(u, v);
        }
        if let Some(&v) = points.get(&(x, y - 1)) {
            uf.union(u, v);
        }
        if let Some(&v) = points.get(&(x, y + 1)) {
            uf.union(u, v);
        }
        if let Some(&v) = points.get(&(x + 1, y)) {
            uf.union(u, v);
        }
        if let Some(&v) = points.get(&(x + 1, y + 1)) {
            uf.union(u, v);
        }
    }

    let mut leaders = HashSet::new();
    for i in 0..n {
        let v = uf.find(i);
        leaders.insert(v);
    }

    println!("{}", leaders.len());
}

pub struct UnionFind {
    parents: Vec<isize>,
    ranks: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parents: vec![-1; n],
            ranks: vec![1; n],
        }
    }

    pub fn union(&mut self, mut x: usize, mut y: usize) {
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
        self.parents[x] = y as isize;
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parents[x] < 0 {
            return x;
        }
        self.parents[x] = self.find(self.parents[x] as usize) as isize;

        return self.parents[x] as usize;
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        return self.find(x) == self.find(y);
    }
}
