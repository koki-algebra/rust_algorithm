use proconio::{fastout, input};

#[derive(PartialEq, Clone)]
enum Color {
    Unknown,
    Black,
    White,
}

#[fastout]
fn main() {
    input! {
        n: usize,
        edge: [(usize, usize); n-1],
    }

    let mut graph = vec![Vec::new(); n];
    for (a, b) in edge {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    let s = 0;
    let mut colors = vec![Color::Unknown; n];
    colors[s] = Color::White;

    dfs(&graph, &mut colors, s);

    let mut whites = Vec::new();
    let mut blacks = Vec::new();
    for (v, color) in colors.iter().enumerate() {
        if *color == Color::White {
            whites.push(v + 1);
        } else if *color == Color::Black {
            blacks.push(v + 1);
        }
    }

    if whites.len() > blacks.len() {
        for i in 0..n / 2 {
            print!("{} ", whites[i]);
        }
    } else {
        for i in 0..n / 2 {
            print!("{} ", blacks[i]);
        }
    }
    println!();
}

fn dfs(graph: &Vec<Vec<usize>>, colors: &mut Vec<Color>, cur: usize) {
    for nv in graph[cur].iter() {
        if colors[*nv] == Color::Unknown {
            if colors[cur] == Color::White {
                colors[*nv] = Color::Black;
            } else if colors[cur] == Color::Black {
                colors[*nv] = Color::White;
            }
            dfs(graph, colors, *nv);
        }
    }
}
