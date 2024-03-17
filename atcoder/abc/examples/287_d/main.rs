use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    // pre[i]: S の先頭 i 文字が T の先頭 i 文字と一致するかどうか
    let mut pre = vec![false; t.len() + 1];
    pre[0] = true;
    // suf[i]: T の末尾　 i 文字と T の末尾 i 文字が一致するかどうか
    let mut suf = vec![false; t.len() + 1];
    suf[0] = true;

    for i in 1..=t.len() {
        pre[i] = (s[i - 1] == t[i - 1] || t[i - 1] == '?' || s[i - 1] == '?') && pre[i - 1];
        suf[i] =
            (t[t.len() - i] == s[s.len() - i] || t[t.len() - i] == '?' || s[s.len() - i] == '?')
                && suf[i - 1];
    }

    for x in 0..=t.len() {
        if pre[x] && suf[t.len() - x] {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}
