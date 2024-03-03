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
    let digits = count_binary_digits(exp);

    for i in 0..digits {
        if exp >> i & 1 == 1 {
            ret = ret * x % m;
        }

        x = x * x % m;
    }

    ret
}

// Calculate the number of digits when representing a decimal integer in binary. O(log(n))
fn count_binary_digits(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut cnt = 0;
    let mut num = n;

    while num > 0 {
        num >>= 1;
        cnt += 1;
    }

    cnt
}
