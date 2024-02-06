use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut cnt = vec![0; n + 1];
    for i in 2..=n {
        if cnt[i] == 0 {
            let mut j = i;
            while j <= n {
                cnt[j] += 1;
                j += i;
            }
        }
    }

    let mut ans = 0;
    for &v in cnt.iter() {
        if v >= k {
            ans += 1;
        }
    }

    println!("{}", ans);
}
