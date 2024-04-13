use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        field: [Chars; h],
    }

    let mut seen = vec![vec![false; w]; h];

    let mut ans = 0;
    for row in 0..h {
        for col in 0..w {
            chmax(&mut ans, dfs(&field, &mut seen, row, col, row, col));
        }
    }

    if ans <= 2 {
        println!("{}", -1);
        return;
    }

    println!("{}", ans);
}

fn dfs(
    field: &Vec<Vec<char>>,
    seen: &mut Vec<Vec<bool>>,
    s_row: usize,
    s_col: usize,
    p_row: usize,
    p_col: usize,
) -> isize {
    if p_row == s_row && p_col == s_col && seen[p_row][p_col] {
        return 0;
    }

    seen[p_row][p_col] = true;

    let h = field.len();
    let w = field[0].len();

    let dr = [-1, 0, 1, 0];
    let dc = [0, 1, 0, -1];

    let mut ret = -isize::MAX;

    for dir in 0..4 {
        let next_row = p_row as isize + dr[dir];
        let next_col = p_col as isize + dc[dir];
        if next_row < 0 || next_col < 0 {
            continue;
        }

        let next_row = next_row as usize;
        let next_col = next_col as usize;

        if next_row < h && next_col < w && field[next_row][next_col] == '.' {
            if !(next_row == s_row && next_col == s_col) && seen[next_row][next_col] {
                continue;
            }

            let v = dfs(field, seen, s_row, s_col, next_row, next_col);
            chmax(&mut ret, v + 1);
        }
    }

    seen[p_row][p_col] = false;

    ret
}

fn chmax(a: &mut isize, b: isize) -> bool {
    if *a < b {
        *a = b;
        return true;
    }

    false
}
