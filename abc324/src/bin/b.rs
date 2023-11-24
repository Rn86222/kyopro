use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u128,
    }
    let mut n = n;
    while n % 2 == 0 {
        n /= 2;
    }
    while n % 3 == 0 {
        n /= 3;
    }
    if n != 1 {
        println!("No");
    } else {
        println!("Yes");
    }
}
