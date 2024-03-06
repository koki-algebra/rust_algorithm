use std::collections::HashMap;

pub fn prime_factorize(n: usize) -> HashMap<usize, usize> {
    // prime factors
    let mut factors = HashMap::new();
    // remainder
    let mut rem = n;

    let mut p = 2;
    while p * p <= n {
        if rem % p == 0 {
            let mut exp = 0;
            while rem % p == 0 {
                exp += 1;
                rem /= p;
            }

            factors.insert(p, exp);
        }

        p += 1;
    }

    if rem != 1 {
        factors.insert(rem, 1);
    }

    factors
}
