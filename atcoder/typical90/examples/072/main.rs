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
    sx: usize,
    sy: usize,
    px: usize,
    py: usize,
) -> isize {
    if sx == px && sy == py && seen[px][py] {
        return 0;
    }

    seen[px][py] = true;
    let mut ret = -isize::MAX;

    let dx: [i32; 4] = [0, 1, 0, -1];
    let dy: [i32; 4] = [1, 0, -1, 0];

    for i in 0..4 {
        let nx = (px as i32 + dx[i]) as usize;
        let ny = (py as i32 + dy[i]) as usize;

        if nx < c.len() && ny < c[0].len() && c[nx][ny] != '#' {
            if (nx != sx || ny != sy) && seen[nx][ny] {
                continue;
            }

            let v = dfs(seen, c, sx, sy, nx, ny);
            chmax(&mut ret, v + 1);
        }
    }

    seen[px][py] = false;

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
