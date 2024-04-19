use std::collections::VecDeque;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut deque = VecDeque::new();

    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                x: usize,
                c: usize,
            }
            deque.push_back((x, c));
        } else {
            input! {
                c: usize,
            }

            let mut ans = 0;
            let mut rem = c;
            while rem > 0 {
                if let Some((v, cnt)) = deque.pop_front() {
                    if cnt >= rem {
                        if cnt != rem {
                            deque.push_front((v, cnt - rem));
                        }
                        ans += v * rem;
                        rem = 0;
                        println!("{}", ans);
                    } else {
                        ans += v * cnt;
                        rem -= cnt;
                    }
                }
            }
        }
    }
}
