use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        roads: [(usize, usize, isize); m],
    }

    let mut graph = vec![Vec::<(isize, usize)>::new(); n];
    for &(a, b, c) in roads.iter() {
        graph[a - 1].push((c, b - 1));
        graph[b - 1].push((c, a - 1));
    }

    let start = 0;
    let dist1 = dijkstra(&graph, start);
    let dist2 = dijkstra(&graph, n - 1);

    for k in 0..n {
        println!("{}", dist1[k] + dist2[k]);
    }
}

fn dijkstra(graph: &Vec<Vec<(isize, usize)>>, start: usize) -> Vec<isize> {
    // Shortest distance from start node
    let mut dist = vec![isize::MAX; graph.len()];
    dist[start] = 0;

    // min-heap
    let mut heap = BinaryHeap::<Reverse<(isize, usize)>>::new();
    heap.push(Reverse((dist[start], start)));

    while let Some(Reverse((now_cost, now))) = heap.pop() {
        if dist[now] < now_cost {
            continue;
        }

        for &(cost, next) in graph[now].iter() {
            let next_cost = now_cost + cost;
            if chmin(&mut dist[next], next_cost) {
                heap.push(Reverse((next_cost, next)));
            }
        }
    }

    dist
}

fn chmin(a: &mut isize, b: isize) -> bool {
    if *a > b {
        *a = b;
        true
    } else {
        false
    }
}
