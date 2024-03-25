use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
    }

    // 差分が 0 でないかもしれない場所
    let mut added_indeces = Vec::new();
    for i in 0..n {
        added_indeces.push(i);
    }

    let mut base = 0;

    for _i in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                x: usize,
            }
            // 差分が 0 でないところを消去
            while let Some(i) = added_indeces.pop() {
                a[i] = 0;
            }
            base = x;
        } else if t == 2 {
            input! {
                i: usize,
                x: usize,
            }
            a[i - 1] += x;
            added_indeces.push(i - 1);
        } else if t == 3 {
            input! {
                i: usize,
            }
            println!("{}", base + a[i - 1]);
        }
    }
}
