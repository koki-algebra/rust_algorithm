use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32
    }

    for bit in 0..1 << n {
        let mut s = String::from("");
        let mut cnt = 0;

        for i in 0..n {
            if bit >> i & 1 == 1 {
                s = format!("{}{}", ")", s);
                cnt += 1;
            } else {
                s = format!("{}{}", "(", s);
                cnt -= 1;
            }

            if cnt < 0 {
                break;
            }
        }

        if cnt == 0 {
            println!("{s}");
        }
    }
}
