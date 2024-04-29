use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut t = Vec::new();
    for &c in s.iter() {
        let v = match c {
            'a' => 1,
            't' => 2,
            'c' => 3,
            'o' => 4,
            'd' => 5,
            'e' => 6,
            'r' => 7,
            _ => 0,
        };

        t.push(v);
    }

    let mut ans = 0;
    // bubble sort
    for i in 0..6 {
        for j in (i + 1..7).rev() {
            if t[j - 1] > t[j] {
                t.swap(j - 1, j);
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
