// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: i64,
        a: [i64; n],
    }
    let mut answer = 0;
    for i in 0..n {
        if a[i] >= l {
            answer += 1;
        }
    }
    println!("{}", answer);
}
