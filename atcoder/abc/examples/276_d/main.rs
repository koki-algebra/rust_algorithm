use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
    }

    let mut g = 0;
    for &v in a.iter() {
        g = gcd(g, v);
    }

    let mut ans = 0;
    for i in 0..n {
        a[i] /= g;
        while a[i] % 2 == 0 {
            a[i] /= 2;
            ans += 1;
        }
        while a[i] % 3 == 0 {
            a[i] /= 3;
            ans += 1;
        }
        if a[i] != 1 {
            println!("{}", -1);
            return;
        }
    }

    println!("{}", ans);
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        return a;
    }

    return gcd(b, a % b);
}
