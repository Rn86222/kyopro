// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ass: [u128; n]
    }
    for a in ass.clone() {
        if a == 0 {
            println!("0");
            return;
        }
    }
    let mut ans = 1;
    for a in ass {
        if ans * a > 10_u128.pow(18) {
            println!("-1");
            return;
        }
        ans *= a;
    }
    println!("{}", ans);
}
