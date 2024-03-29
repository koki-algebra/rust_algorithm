use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        field: [Chars; n],
    }

    let mut a = vec![Vec::new(); n];
    for i in 0..n {
        for j in 0..n {
            a[i].push(field[i][j].to_digit(10).unwrap() as isize);
        }
    }

    let mut ans = 0;

    // 上, 右上, 右, 右下, 下, 左下, 左, 左上
    let dx: Vec<isize> = vec![0, 1, 1, 1, 0, -1, -1, -1];
    let dy: Vec<isize> = vec![1, 1, 0, -1, -1, -1, 0, 1];

    let size = n as isize;

    for i in 0..n {
        for j in 0..n {
            for k in 0..8 {
                let mut now = 0;
                let mut x = i as isize;
                let mut y = j as isize;
                for _ in 0..n {
                    now *= 10;
                    now += a[x as usize][y as usize];
                    x += dx[k];
                    y += dy[k];
                    x %= size;
                    x += size;
                    y %= size;
                    y += size;
                    x %= size;
                    y %= size;
                }
                ans = ans.max(now);
            }
        }
    }

    println!("{}", ans);
}
