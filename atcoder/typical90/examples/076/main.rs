use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let sum: usize = a.iter().sum();
    if sum % 10 != 0 {
        println!("No");
        return;
    }

    let target = sum / 10;

    let mut b = vec![0; 2 * n];
    for i in 0..n {
        b[i] = a[i];
        b[i + n] = a[i];
    }

    let mut right = 0;
    let mut val = 0;
    for left in 0..2 * n {
        while right < 2 * n {
            val += b[right];
            right += 1;
            if val == target {
                println!("Yes");
                return;
            } else if val > target {
                break;
            }
        }
        val -= b[left];
    }

    println!("No");
}
