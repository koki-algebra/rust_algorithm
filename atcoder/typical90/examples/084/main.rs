use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut d = vec![0];
    let mut prev = s[0];
    for &c in s.iter() {
        if c != prev {
            d.push(1);
            prev = c;
        } else {
            let l = d.len();
            d[l - 1] += 1;
        }
    }

    let complement: usize = d.iter().map(|v| v * (v + 1) / 2).sum();

    println!("{}", n * (n + 1) / 2 - complement);
}
