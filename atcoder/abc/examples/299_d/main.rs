use std::io::{stdin, BufReader};

use proconio::{input, source::line::LineSource};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        n: usize,
    }

    let mut left = 1;
    let mut right = n;
    while right - left > 1 {
        let mid = left + (right - left) / 2;
        println!("? {}", mid);
        input! {
            from &mut source,
            s: usize,
        }
        if s == 0 {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("! {}", left);
}
