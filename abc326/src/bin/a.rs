// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i128,
        y: i128,
    }
    if x < y {
        if y - x > 2 {
            println!("No");
        } else {
            println!("Yes");
        }
    } else {
        if x - y > 3 {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
