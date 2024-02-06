use std::collections::HashMap;

pub fn prime_factorize(n: usize) -> HashMap<usize, usize> {
    let mut n = n;
    let mut ret = HashMap::new();
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            let mut e = 0;
            while n % p == 0 {
                e += 1;
                n /= p;
            }

            ret.insert(p, e);
        }

        p += 1;
    }

    if n != 1 {
        ret.insert(n, 1);
    }

    return ret;
}
