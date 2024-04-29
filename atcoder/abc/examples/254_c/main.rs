use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut b = vec![Vec::new(); k];
    for i in 0..n {
        b[i % k].push(a[i]);
    }
    for i in 0..k {
        // sort by descending order
        b[i].sort_by(|a, b| b.cmp(a));
    }

    let mut na = Vec::new();
    for i in 0..n {
        if let Some(v) = b[i % k].pop() {
            na.push(v);
        }
    }

    for i in 0..n - 1 {
        if na[i] > na[i + 1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
