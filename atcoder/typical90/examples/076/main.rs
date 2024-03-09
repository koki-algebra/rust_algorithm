use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut b = vec![0; 2 * n];
    for i in 0..n {
        b[i] = a[i];
        b[i + n] = a[i];
    }

    let mut sums = vec![0; 2 * n + 1];
    for i in 0..2 * n {
        sums[i + 1] += sums[i] + b[i];
    }

    if sums[n] % 10 != 0 {
        println!("No");
        return;
    }
    let target = sums[n] / 10;

    let mut left = 0;
    let mut right = 1;
    while left <= 2 * n {
        while right <= 2 * n && sums[right] - sums[left] <= target {
            if sums[right] - sums[left] == target {
                println!("Yes");
                return;
            }
            right += 1;
        }
        left += 1;
    }

    println!("No");
}
