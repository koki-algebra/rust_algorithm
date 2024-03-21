use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
        cases: [usize; t],
    }

    for &n in cases.iter() {
        if let Some((p, q)) = solve(n) {
            println!("{} {}", p, q);
        }
    }
}

fn solve(n: usize) -> Option<(usize, usize)> {
    let mut i = 2;
    while i * i * i <= n {
        if n % i == 0 {
            if (n / i) % i == 0 {
                let p = i;
                let q = n / i / i;
                return Some((p, q));
            } else {
                let q = i;
                let p = ((n / q) as f64).sqrt() as usize;
                return Some((p, q));
            }
        }
        i += 1;
    }

    None
}
