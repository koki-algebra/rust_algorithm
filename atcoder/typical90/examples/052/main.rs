use proconio::input;

const MOD: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        a: [[usize; 6]; n],
    }

    let mut r = Vec::new();
    for i in 0..n {
        let mut sum = 0;
        for j in 0..6 {
            sum += a[i][j]
        }
        r.push(sum);
    }

    let mut ans = 1;
    for &v in r.iter() {
        ans *= v;
        ans %= MOD;
    }

    println!("{}", ans);
}
