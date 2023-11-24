use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }

    let mut max = std::i64::MIN;
    for i in 0..n {
        if max < a[i] {
            max = a[i];
        }
    }

    let mut ans = 0;
    for i in 0..n {
        if a[i] < max && ans < a[i] {
            ans = a[i];
        }
    }

    println!("{}", ans);
}
