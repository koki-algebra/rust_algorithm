use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut d = vec![1];
    let mut prev = s[0];
    for i in 1..n {
        if s[i] != prev {
            d.push(1);
            prev = s[i];
        } else {
            let l = d.len();
            d[l - 1] += 1;
        }
    }

    let mut ret = 0;
    for &v in d.iter() {
        ret += v * (v + 1) / 2;
    }

    let ans = n * (n + 1) / 2 - ret;

    println!("{}", ans);
}
