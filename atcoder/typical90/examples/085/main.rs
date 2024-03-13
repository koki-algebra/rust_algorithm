use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: usize,
    }

    // k の約数を列挙
    let divisors = enumerate_divisors(k);

    let mut ans = 0;
    for i in 0..divisors.len() {
        for j in i..divisors.len() {
            // divisors[i] * divisors[j] が最大 10^24 なのでオーバーフローを避ける
            if k / divisors[i] < divisors[j] {
                continue;
            }
            if k % (divisors[i] * divisors[j]) != 0 {
                continue;
            }
            if divisors[j] <= k / (divisors[i] * divisors[j]) {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

fn enumerate_divisors(n: usize) -> Vec<usize> {
    let mut divisors = Vec::new();
    let mut p = 1;
    while p * p <= n {
        if n % p == 0 {
            divisors.push(p);
            if n / p != p {
                divisors.push(n / p);
            }
        }
        p += 1;
    }

    divisors.sort();
    divisors
}
