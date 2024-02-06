use proconio::{fastout, input};

#[derive(Clone)]
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
        match *color {
            Color::White => {
                whites.push(v + 1);
            }
            Color::Black => {
                blacks.push(v + 1);
            }
            Color::Unknown => continue,
        };
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
        match colors[*nv] {
            Color::Unknown => {
                match colors[cur] {
                    Color::White => {
                        colors[*nv] = Color::Black;
                    }
                    Color::Black => {
                        colors[*nv] = Color::White;
                    }
                    Color::Unknown => (),
                }
                dfs(graph, colors, *nv);
            }
            _ => continue,
        };
    }
}
