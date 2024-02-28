use proconio::input;

const MOD: usize = 100000;

fn main() {
    input! {
        n: usize,
        mut k: isize,
    }

    let mut nexts = vec![0; MOD];
    let mut timestamps: Vec<isize> = vec![-1; MOD];
    for i in 0..MOD {
        nexts[i] = (i + digit_sum(i)) % MOD;
    }

    let mut pos = n;
    let mut index = 0;
    while timestamps[pos] == -1 {
        timestamps[pos] = index;
        pos = nexts[pos];
        index += 1;
    }

    // the timing of entering the cycle
    let find = timestamps[pos];

    // cycle length
    let cycle = index - find;

    if k >= find {
        k = (k - find) % cycle + find;
    }

    for i in 0..MOD {
        if timestamps[i] == k {
            println!("{}", i);
            return;
        }
    }
}

fn digit_sum(mut x: usize) -> usize {
    let mut sum = 0;
    while x != 0 {
        sum += x % 10;
        x /= 10;
    }

    sum
}
