use std::vec;

use proconio::{fastout, input};

type Graph = Vec<Vec<usize>>;

#[fastout]
fn main() {
    input! {
        n: usize,
        edge: [(usize, usize); n-1],
    }

    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];

    for (a, b) in edge {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    let mut dist = vec![-1; n];
    let s = 0; // start node
    dist[s] = 0;

    dfs(&graph, &mut dist, s);

    let max_index = dist
        .iter()
        .enumerate()
        .max_by_key(|&(_, v)| v)
        .map(|(i, _)| i)
        .unwrap();

    dist = vec![-1; n];
    dist[max_index] = 0;
    dfs(&graph, &mut dist, max_index);

    let ans = dist.iter().max().copied().unwrap() + 1;
    println!("{}", ans);
}

fn dfs(graph: &Graph, dist: &mut Vec<isize>, cur: usize) {
    for nv in &graph[cur] {
        if dist[*nv] == -1 {
            dist[*nv] = dist[cur] + 1;
            dfs(graph, dist, *nv);
        }
    }
}
