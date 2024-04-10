use proconio::{fastout, input};

const MOD: usize = 1000000007;

#[fastout]
fn main() {
    input! {
        l: usize,
        r: usize,
    }

    let mut power10 = vec![0; 20];
    power10[0] = 1;
    for i in 0..19 {
        power10[i + 1] = power10[i] * 10;
    }

    let mut ans = 0;
    for i in 1..=19 {
        let vl = l.max(power10[i - 1]);
        let vr = r.min(power10[i] - 1);
        if vl > vr {
            continue;
        }

        let val = (f(vr, MOD) - f(vl - 1, MOD) + MOD) % MOD;
        ans += i * val;
        ans %= MOD;
    }

    println!("{}", ans);
}

fn div(a: usize, b: usize, m: usize) -> usize {
    return a * pow(b, m - 2, m) % m;
}

fn f(n: usize, m: usize) -> usize {
    let v1 = n % m;
    let v2 = (n + 1) % m;
    let v = v1 * v2 % m;
    div(v, 2, m)
}

fn pow(base: usize, exp: usize, m: usize) -> usize {
    let mut ret = 1;
    let mut x = base;
    let mut rem = exp;

    while rem > 0 {
        if rem & 1 == 1 {
            ret = ret * x % m;
        }
        x = x * x % m;
        rem >>= 1;
    }

    ret
}
