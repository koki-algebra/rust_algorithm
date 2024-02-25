use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: isize,
        q: isize,
        mut a: [isize; n],
        queries: [(isize, isize, isize); q],
    }

    let mut top = 0;
    for &(t, x, y) in queries.iter() {
        let x = (x - 1 + top) % n;
        let y = (y - 1 + top) % n;
        match t {
            1 => {
                a.swap(x as usize, y as usize);
            }
            2 => {
                top = (top + n - 1) % n;
            }
            3 => {
                println!("{}", a[x as usize]);
            }
            _ => (),
        };
    }
}
