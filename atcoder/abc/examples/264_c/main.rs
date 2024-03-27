use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h1: usize,
        w1: usize,
        a: [[usize; w1]; h1],
        h2: usize,
        w2: usize,
        b: [[usize; w2]; h2],
    }

    for rows in 0..1 << h1 {
        let mut selected_rows = Vec::new();
        for i in 0..h1 {
            if rows >> i & 1 == 1 {
                selected_rows.push(i);
            }
        }
        if selected_rows.len() != h2 {
            continue;
        }
        for cols in 0..1 << w1 {
            let mut selected_cols = Vec::new();
            for i in 0..w1 {
                if cols >> i & 1 == 1 {
                    selected_cols.push(i);
                }
            }
            if selected_cols.len() != w2 {
                continue;
            }

            let mut valid = true;
            for i in 0..h2 {
                for j in 0..w2 {
                    if a[selected_rows[i]][selected_cols[j]] != b[i][j] {
                        valid = false;
                        break;
                    }
                }
            }
            if valid {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
