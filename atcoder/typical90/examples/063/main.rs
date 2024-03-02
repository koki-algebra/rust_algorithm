use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        p: [[isize; w]; h],
    }

    let mut ans = -1;
    for bit in 0..1 << h {
        // key: number, value: count
        let mut m = HashMap::new();

        for col in 0..w {
            let mut num = -1;
            let mut valid = true;
            for row in 0..h {
                if bit >> row & 1 == 1 {
                    if num == -1 {
                        num = p[row][col];
                    } else if p[row][col] != num {
                        valid = false;
                    }
                }
            }

            if valid && num != -1 {
                *m.entry(num).or_insert(0) += 1;
            }
        }

        // number of columns
        let cnt_w = maximum_same(m);

        // number of rows
        let mut cnt_h = 0;
        for row in 0..h {
            if bit >> row & 1 == 1 {
                cnt_h += 1;
            }
        }

        chmax(&mut ans, cnt_h * cnt_w);
    }

    println!("{}", ans);
}

fn maximum_same(m: HashMap<isize, isize>) -> isize {
    let mut ret = -1;
    for (_key, &value) in m.iter() {
        chmax(&mut ret, value);
    }

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
