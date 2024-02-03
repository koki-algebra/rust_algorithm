use std::f64::consts::PI;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        e: [f64; q]
    }

    // angular frequency
    let omega = 2.0 * PI / t;
    // amplitude
    let amp = l / 2.0;

    for i in 0..q {
        // y-coordinate after e[i] minutes
        let yi = -amp * (omega * e[i]).sin();
        // z-coordinate after e[i] minutes
        let zi = amp * (1.0 - (omega * e[i]).cos());

        // horizontal distance
        let dy = y - yi;
        let dist = (x * x + dy * dy).sqrt();

        println!("{}", (zi / dist).atan().to_degrees());
    }
}
