use std::collections::{HashMap, HashSet};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ladders: [(usize, usize); n],
    }

    let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut seen = HashMap::new();

    for &(a, b) in ladders.iter() {
        graph.entry(a).or_insert_with(HashSet::new).insert(b);
        graph.entry(b).or_insert_with(HashSet::new).insert(a);
        seen.insert(a, false);
        seen.insert(b, false);
    }

    let start = 1;
    let mut ans = 1;
    dfs(&graph, &mut seen, start, &mut ans);

    println!("{}", ans);
}

fn dfs(
    graph: &HashMap<usize, HashSet<usize>>,
    seen: &mut HashMap<usize, bool>,
    start: usize,
    max_key: &mut usize,
) {
    seen.insert(start, true);

    if let Some(s) = graph.get(&start) {
        for &nv in s.iter() {
            if !seen[&nv] {
                chmax(max_key, nv);
                dfs(graph, seen, nv, max_key);
            }
        }
    }
}

fn chmax(a: &mut usize, b: usize) -> bool {
    if *a < b {
        *a = b;
        return true;
    }

    false
}
