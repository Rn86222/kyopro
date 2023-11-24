// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let max = a.iter().max().unwrap();
    let k = *max;
    let mut xor_sum = 0;
    for i in 0..n {
        xor_sum ^= a[i] % (k + 1);
    }
    if xor_sum != 0 {
        println!("-1");
        return;
    }
    let mut left = 0;
    let mut right = *max + 1;

    while right - left > 1 {
        let k = left + (right - left) / 2;
        let mut xor_sum = 0;
        for i in 0..n {
            xor_sum ^= a[i] % (k + 1);
        }
        if xor_sum != 0 {
            left = k;
        } else {
            right = k;
        }
    }
    let ans = left;
    if ans == 0 {
        println!("0");
    } else {
        println!("{}", ans);
    }
}
