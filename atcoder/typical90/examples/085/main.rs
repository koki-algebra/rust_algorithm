use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: usize,
    }

    let divisors = get_divisors(k);

    let mut ans = 0;
    for i in 0..divisors.len() {
        for j in i..divisors.len() {
            let a = divisors[i];
            let b = divisors[j];
            if k / a < b {
                continue;
            }
            if k % (a * b) != 0 {
                continue;
            }
            if b <= k / (a * b) {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

fn get_divisors(n: usize) -> Vec<usize> {
    let mut ret = Vec::new();
    let mut p = 1;
    while p * p <= n {
        if n % p == 0 {
            ret.push(p);
            let rem = n / p;
            if rem != p {
                ret.push(rem);
            }
        }
        p += 1;
    }

    ret.sort();
    ret
}
