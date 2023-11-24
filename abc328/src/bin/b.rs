// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ds: [usize; n],
    }

    let mut ans = 0;

    for i in 1..=n {
        if i <= 9 {
            if ds[i - 1] >= i {
                ans += 1;
            }
            if ds[i - 1] >= 11 * i {
                ans += 1;
            }
        }
        if i >= 10 && i <= 99 {
            if i / 10 == i % 10 {
                if ds[i - 1] >= i {
                    ans += 1;
                }
                if ds[i - 1] >= i % 10 {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
