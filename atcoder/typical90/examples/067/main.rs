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

fn to_decimal(n: &String, base: u32) -> usize {
    let mut ret = 0;

    for c in n.chars() {
        let digit = c.to_digit(base).unwrap() as usize;
        ret = ret * 8 + digit;
    }

    ret
}

fn parse_decimal(mut n: usize, base: usize) -> String {
    if n == 0 {
        return String::from("0");
    }

    let mut ret = String::from("");
    while n != 0 {
        let c = (n % base).to_string();
        ret.insert_str(0, &c);
        n /= base;
    }

    ret
}
