use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: String,
        k: usize,
    }

    for _i in 0..k {
        n = parse_decimal(to_decimal(&n, 8), 9);
        n = n.replace("8", "5");
    }

    println!("{}", n);
}

fn to_decimal(n: &str, base: usize) -> usize {
    let mut ret = 0;
    let mut x = 1;
    for c in n.chars().rev() {
        if let Some(digit) = c.to_digit(base as u32) {
            ret += x * digit as usize;
        }
        x *= base;
    }

    ret
}

fn parse_decimal(n: usize, base: usize) -> String {
    if n == 0 {
        return "0".to_string();
    }

    let mut num = n;
    let mut ret = String::new();
    while num > 0 {
        let c = (num % base).to_string();
        ret = format!("{}{}", c, ret);
        num /= base;
    }

    ret
}
