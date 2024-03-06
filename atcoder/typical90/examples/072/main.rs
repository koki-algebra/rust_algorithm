use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    let mut seen = vec![vec![false; w]; h];
    let mut ans = -1;

    for i in 0..h {
        for j in 0..w {
            chmax(&mut ans, dfs(&mut seen, &c, i, j, i, j));
        }
    }

    if ans <= 2 {
        ans = -1;
    }

    println!("{}", ans);
}

fn dfs(
    seen: &mut Vec<Vec<bool>>,
    c: &Vec<Vec<char>>,
    s_row: usize,
    s_col: usize,
    p_row: usize,
    p_col: usize,
) -> isize {
    if s_row == p_row && s_col == p_col && seen[p_row][p_col] {
        return 0;
    }

    let h = c.len();
    let w = c[0].len();

    let d_row: [i32; 4] = [0, 1, 0, -1];
    let d_col: [i32; 4] = [1, 0, -1, 0];

    seen[p_row][p_col] = true;
    let mut ret = -isize::MAX;

    for i in 0..4 {
        let next_row = (p_row as i32 + d_row[i]) as usize;
        let next_col = (p_col as i32 + d_col[i]) as usize;

        if next_row < h && next_col < w && c[next_row][next_col] == '.' {
            if (next_row != s_row || next_col != s_col) && seen[next_row][next_col] {
                continue;
            }

            let v = dfs(seen, c, s_row, s_col, next_row, next_col);
            chmax(&mut ret, v + 1);
        }
    }

    seen[p_row][p_col] = false;

    ret
}

fn chmax(a: &mut isize, b: isize) -> bool {
    if *a < b {
        *a = b;
        true
    } else {
        false
    }
}
