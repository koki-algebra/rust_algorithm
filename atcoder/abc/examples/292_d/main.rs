use std::mem::swap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }

    let mut uf = UnionFind::new(n);
    for &(u, v) in edges.iter() {
        uf.union(u - 1, v - 1);
    }

    let mut num_vertices = vec![0; n];
    let mut num_edges = vec![0; n];

    for i in 0..n {
        num_vertices[uf.find(i)] += 1;
    }
    for &(u, _v) in edges.iter() {
        num_edges[uf.find(u - 1)] += 1;
    }

    for i in 0..n {
        if num_vertices[i] != num_edges[i] {
            println!("No");
            return;
        }
    }

    println!("Yes")
}

pub struct UnionFind {
    parents: Vec<i32>,
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
        self.parents[x] = y as i32;
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parents[x] < 0 {
            return x;
        }
        self.parents[x] = self.find(self.parents[x] as usize) as i32;

        return self.parents[x] as usize;
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        return self.find(x) == self.find(y);
    }
}
