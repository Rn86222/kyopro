use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: i128,
        M: i128,
        P: i128
    }
    if N < M {
        println!("0");
    } else {
        println!("{}", (N - M) / P + 1);
    }
}
