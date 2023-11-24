use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        b: u128,
    }
    let mut powers = vec![];
    for i in 1..=16 {
        let i: u128 = i;
        powers.push((i, i.pow(i as u32)));
    }
    for (i, p) in powers {
        if p == b {
            println!("{}", i);
            return;
        }
    }
    println!("{}", -1);
}
