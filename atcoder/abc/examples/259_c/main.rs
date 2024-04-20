use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let a = rle(&s);
    let b = rle(&t);
    if a.len() != b.len() {
        println!("No");
        return;
    }

    for (&(c1, v1), &(c2, v2)) in a.iter().zip(b.iter()) {
        if c1 != c2 || v1 > v2 || v1 < v2 && v1 == 1 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

// Run Length Encoding
fn rle<T: Copy + PartialEq>(s: &[T]) -> Vec<(T, usize)> {
    let mut ret = Vec::new();
    let n = s.len();

    if n == 0 {
        return ret;
    }
    if n == 1 {
        ret.push((s[0], 1));
        return ret;
    }

    let mut cnt = 1;
    for i in 1..n {
        if s[i] != s[i - 1] {
            ret.push((s[i - 1], cnt));
            cnt = 0;
        }
        cnt += 1;
    }

    ret.push((s[n - 1], cnt));

    ret
}
