use std::{cmp::Reverse, collections::BinaryHeap};

// State is a tuple consisting of (cost, node)
type State = (isize, usize);

type Graph = Vec<Vec<State>>;

pub fn dijkstra(graph: &Graph, start: usize) -> Vec<isize> {
    // Shortest distance from start node
    let mut dist = vec![isize::MAX; graph.len()];
    dist[start] = 0;

    let mut heap = BinaryHeap::<Reverse<State>>::new();
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
