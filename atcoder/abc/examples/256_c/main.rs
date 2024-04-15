use proconio::{fastout, input};

const MAX: isize = 28;

#[fastout]
fn main() {
    input! {
        h1: isize,
        h2: isize,
        h3: isize,
        w1: isize,
        w2: isize,
        w3: isize,
    }

    let mut ans = 0;
    for s11 in 1..=MAX {
        for s12 in 1..=MAX {
            let s13 = h1 - s11 - s12;
            if s13 < 1 {
                continue;
            }
            for s21 in 1..=MAX {
                let s31 = w1 - s11 - s21;
                if s31 < 1 {
                    continue;
                }
                for s22 in 1..=MAX {
                    let s23 = h2 - s21 - s22;
                    let s32 = w2 - s12 - s22;
                    let s33 = h3 - s31 - s32;
                    if s23 < 1 || s32 < 1 || s33 < 1 || s33 != w3 - s13 - s23 {
                        continue;
                    }
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
