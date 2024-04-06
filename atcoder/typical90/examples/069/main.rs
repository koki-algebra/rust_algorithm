use proconio::{fastout, input};

const MOD: usize = 1000000007;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }

    if k == 1 {
        if n == 1 {
            println!("{}", 1);
        } else {
            println!("{}", 0);
        }
    } else if n == 1 {
        println!("{}", k % MOD);
    } else if n == 2 {
        println!("{}", k * (k - 1) % MOD);
    } else {
        println!("{}", k * (k - 1) % MOD * pow(k - 2, n - 2, MOD) % MOD);
    }
}

fn pow(base: usize, exp: usize, m: usize) -> usize {
    let mut ret = 1;
    let mut x = base;

    let mut num = exp;
    while num > 0 {
        if num & 1 == 1 {
            ret = ret * x % m;
        }
        x = x * x % m;
        num >>= 1;
    }

    ret
}
