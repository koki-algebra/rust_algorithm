use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        p: [[usize; w]; h],
    }

    let mut ans = 0;

    for bit in 1..1 << h {
        let mut rows = Vec::new();
        for row in 0..h {
            if bit >> row & 1 == 1 {
                rows.push(row);
            }
        }

        let mut nums = HashMap::new();

        for col in 0..w {
            let mut is_valid = true;
            let num = p[rows[0]][col];
            for &row in rows.iter() {
                if p[row][col] != num {
                    is_valid = false;
                    break;
                }
            }
            if is_valid {
                *nums.entry(num).or_insert(0) += 1;
            }
        }

        if let Some(&col_cnt) = nums.values().max() {
            ans = ans.max(rows.len() * col_cnt);
        }
    }

    println!("{}", ans);
}
