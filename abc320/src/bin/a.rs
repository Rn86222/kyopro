use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u32,
        b: u32
    }
    println!("{}", a.pow(b) + b.pow(a));
}
