use proconio::input;

const MOD: usize = 46;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }

    let mut m_a: Vec<isize> = vec![0; MOD];
    let mut m_b: Vec<isize> = vec![0; MOD];
    let mut m_c: Vec<isize> = vec![0; MOD];

    for &v in a.iter() {
        m_a[v % MOD] += 1;
    }
    for &v in b.iter() {
        m_b[v % MOD] += 1;
    }
    for &v in c.iter() {
        m_c[v % MOD] += 1;
    }

    let mut ans = 0;
    for x in 0..MOD {
        for y in 0..MOD {
            for z in 0..MOD {
                if (x + y + z) % MOD == 0 {
                    ans += m_a[x] * m_b[y] * m_c[z];
                }
            }
        }
    }

    println!("{}", ans);
}
