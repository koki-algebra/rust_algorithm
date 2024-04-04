use proconio::{fastout, input};

const MOD: usize = 46;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }

    let mut a_mod: Vec<usize> = vec![0; MOD];
    let mut b_mod: Vec<usize> = vec![0; MOD];
    let mut c_mod: Vec<usize> = vec![0; MOD];
    for i in 0..n {
        a_mod[a[i] % MOD] += 1;
        b_mod[b[i] % MOD] += 1;
        c_mod[c[i] % MOD] += 1;
    }

    let mut ans = 0;
    for i in 0..MOD {
        for j in 0..MOD {
            for k in 0..MOD {
                if (i + j + k) % MOD == 0 {
                    ans += a_mod[i] * b_mod[j] * c_mod[k];
                }
            }
        }
    }

    println!("{}", ans);
}
