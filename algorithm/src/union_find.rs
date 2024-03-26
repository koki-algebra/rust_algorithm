use std::mem::swap;

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
