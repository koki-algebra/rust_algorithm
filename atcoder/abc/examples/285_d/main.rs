use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        changes: [(String, String); n],
    }

    let mut usernames: HashMap<&String, usize> = HashMap::new();
    let mut name_index = 0;
    for (s, t) in changes.iter() {
        if !usernames.contains_key(&s) {
            usernames.insert(s, name_index);
            name_index += 1;
        }
        if !usernames.contains_key(&t) {
            usernames.insert(t, name_index);
            name_index += 1;
        }
    }

    let mut graph = vec![Vec::<usize>::new(); usernames.len()];
    for (s, t) in changes.iter() {
        graph[usernames[s]].push(usernames[t]);
    }

    let mut seen = vec![false; usernames.len()];
    let mut finished = vec![false; usernames.len()];
    for i in 0..usernames.len() {
        if !finished[i] {
            if dfs(i, &graph, &mut seen, &mut finished) {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}

// サイクルの存在判定
fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, finished: &mut Vec<bool>) -> bool {
    seen[v] = true;
    for &nv in graph[v].iter() {
        if finished[nv] {
            continue;
        }
        if seen[nv] {
            return true;
        }
        if dfs(nv, graph, seen, finished) {
            return true;
        }
    }

    finished[v] = true;

    false
}
