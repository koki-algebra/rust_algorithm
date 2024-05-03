use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    const MAX_P: usize = 1000005;
    let primes = get_primes(MAX_P);

    let mut ans = 0;

    let mut q = primes.len() - 1;
    for p in 0..primes.len() {
        while p < q && f(primes[p], primes[q]) > n {
            q -= 1;
        }
        if p >= q {
            break;
        }

        ans += q - p;
    }

    println!("{}", ans);
}

fn get_primes(n: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    let mut is_primes = vec![true; n + 1];
    is_primes[0] = false;
    is_primes[1] = false;

    for p in 2..=n {
        if !is_primes[p] {
            continue;
        }

        primes.push(p);

        let mut q = p * 2;
        while q <= n {
            is_primes[q] = false;
            q += p;
        }
    }

    primes
}

fn f(p: usize, q: usize) -> usize {
    let v = q as f64;
    if p as f64 * v * v * v > usize::MAX as f64 {
        return usize::MAX;
    }

    p * q * q * q
}
