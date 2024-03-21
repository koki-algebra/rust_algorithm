use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars
    }

    let n = s.len();
    let mut v = vec![Vec::<char>::new(); n];
    let mut used = vec![false; 256];
    let mut now = 0;
    for i in 0..n {
        if s[i] == '(' {
            now += 1;
        } else if s[i] == ')' {
            for &c in v[now].iter() {
                used[c as usize] = false;
            }
            v[now].clear();
            now -= 1;
        } else {
            if used[s[i] as usize] {
                println!("No");
                return;
            }
            v[now].push(s[i]);
            used[s[i] as usize] = true;
        }
    }

    println!("Yes");
}
