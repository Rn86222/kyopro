use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: String,
    }
    let mut s_chars = s.chars().collect_vec();
    s_chars.sort();

    let max = 10_u128.pow(n as u32) - 1;
    let mut i = 0;
    let mut sqrts = vec![];
    loop {
        let sqrt = i * i;
        if sqrt > max {
            break;
        }
        let mut sqrt = format!("{:0>n$}", sqrt).chars().collect_vec();
        sqrt.sort();
        sqrts.push(sqrt);
        i += 1;
    }
    let mut ans = 0;
    for sqrt in sqrts {
        if sqrt == s_chars {
            ans += 1;
        }
    }

    println!("{}", ans);
}
