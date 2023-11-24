// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize
    }
    let mut ns = vec![];
    let mut n = n;
    while n > 0 {
        ns.push(n % 10);
        n /= 10;
    }
    for i in 1..ns.len() {
        if ns[i] <= ns[i - 1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
