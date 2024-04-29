use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    let all_sum = n * (n + 1) / 2;

    let a_cnt = n / a;
    let a_sum = a * a_cnt * (1 + a_cnt) / 2;

    if a == b {
        println!("{}", all_sum - a_sum);
        return;
    }

    let b_cnt = n / b;
    let b_sum = b * b_cnt * (1 + b_cnt) / 2;

    let lcm = a / gcd(a, b) * b;
    let lcm_cnt = n / lcm;
    let lcm_sum = lcm * lcm_cnt * (1 + lcm_cnt) / 2;

    println!("{}", all_sum + lcm_sum - a_sum - b_sum);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}
