use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        edges: [(usize, usize); n-1],
    }

    let mut graph = vec![Vec::new(); n];
    for &(a, b) in edges.iter() {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    let start = 0;

    let mut dist = vec![-1; n];
    dist[start] = 0;
    dfs(&graph, &mut dist, start);

    let start = dist
        .iter()
        .enumerate()
        .max_by_key(|(_, &val)| val)
        .map(|(index, _)| index)
        .unwrap();

    let mut dist = vec![-1; n];
    dist[start] = 0;
    dfs(&graph, &mut dist, start);

    println!("{}", *dist.iter().max().unwrap() + 1);
}

fn dfs(graph: &Vec<Vec<usize>>, dist: &mut Vec<isize>, start: usize) {
    for &nv in graph[start].iter() {
        if dist[nv] == -1 {
            dist[nv] = dist[start] + 1;
            dfs(graph, dist, nv);
        }
    }
}
