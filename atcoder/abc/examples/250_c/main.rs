use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [usize; q],
    }

    let mut a: Vec<usize> = (0..=n).collect();
    let mut pos: Vec<usize> = (0..=n).collect();
    for &x in queries.iter() {
        // the position of x
        let i = pos[x];
        if i == n {
            let v = a[n - 1];
            pos[x] = n - 1;
            pos[v] = n;
            a.swap(n, n - 1);
        } else {
            let v = a[i + 1];
            pos[x] = i + 1;
            pos[v] = i;
            a.swap(i, i + 1);
        }
    }

    for i in 1..=n {
        print!("{} ", a[i]);
    }
    println!();
}
