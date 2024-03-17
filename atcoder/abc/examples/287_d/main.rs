use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    // pre[i]: S と T の先頭 i 文字と一致するかどうか
    let mut pre = vec![false; t.len() + 1];
    pre[0] = true;
    // suf[i]: S と T の末尾 i 文字が一致するかどうか
    let mut suf = vec![false; t.len() + 1];
    suf[0] = true;

    for i in 1..=t.len() {
        pre[i] = is_match(s[i - 1], t[i - 1]) && pre[i - 1];
        suf[i] = is_match(s[s.len() - i], t[t.len() - i]) && suf[i - 1];
    }

    for x in 0..=t.len() {
        if pre[x] && suf[t.len() - x] {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}

fn is_match(c1: char, c2: char) -> bool {
    return c1 == c2 || c1 == '?' || c2 == '?';
}
