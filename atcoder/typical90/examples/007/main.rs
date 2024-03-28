use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
        q: usize,
        queries: [isize; q],
    }

    a.sort();

    for &b in queries.iter() {
        match a.binary_search(&b) {
            Ok(_) => {
                println!("{}", 0);
            }
            Err(index) => {
                if index == n {
                    println!("{}", (a[n - 1] - b).abs());
                } else if index == 0 {
                    println!("{}", (a[0] - b).abs());
                } else {
                    println!("{}", (a[index] - b).abs().min((a[index - 1] - b).abs()));
                }
            }
        }
    }
}
