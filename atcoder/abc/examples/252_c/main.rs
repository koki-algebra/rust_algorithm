use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    // cnts[k][j]: j 番目が k である個数
    let mut cnts = [[0; 10]; 10];

    for i in 0..n {
        for j in 0..10 {
            cnts[s[i][j].to_digit(10).unwrap() as usize][j] += 1;
        }
    }

    let mut ans = 1000;
    for k in 0..10 {
        let mut tmp = 0;
        for j in 0..10 {
            if cnts[k][j] >= 1 {
                tmp = tmp.max(10 * (cnts[k][j] - 1) + j);
            }
        }
        ans = ans.min(tmp);
    }

    println!("{}", ans);
}
