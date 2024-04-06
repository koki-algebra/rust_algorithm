pub fn pow(base: usize, exp: usize, m: usize) -> usize {
    let mut ret = 1;
    let mut x = base;
    let mut num = exp;

    while num > 0 {
        if num & 1 == 1 {
            ret = ret * x % m;
        }
        x = x * x % m;
        num >>= 1;
    }

    ret
}
