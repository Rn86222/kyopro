// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        d: i64,
    }

    let sqrt_d = (d as f64).sqrt() as i64;

    let mut ans = std::i64::MAX;
    for x in 0..sqrt_d + 1 {
        let y = ((d - x * x) as f64).sqrt() as i64;
        if (x * x + y * y - d).abs() < ans {
            ans = (x * x + y * y - d).abs();
        }
        let y = ((d - x * x) as f64).sqrt() as i64 + 1;
        if (x * x + y * y - d).abs() < ans {
            ans = (x * x + y * y - d).abs();
        }
    }
    println!("{}", ans);
}
