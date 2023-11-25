// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: i64,
        r: i64,
        a: [i64; n],
    }

    for i in 0..n {
        if a[i] < l {
            println!("{}", l);
        } else if a[i] > r {
            println!("{}", r);
        } else {
            println!("{}", a[i]);
        }
    }
}
