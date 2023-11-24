use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        s: String,
        lr: [(usize, usize); q],
    }

    let s = s.chars().collect_vec();
    let mut dp = vec![0; n + 1];
    for i in 1..n {
        if s[i - 1] == s[i] {
            dp[i] = dp[i - 1] + 1;
        } else {
            dp[i] = dp[i - 1];
        }
    }

    // println!("{:?}", dp);

    for (l, r) in lr {
        let l = l - 1;
        let r = r - 1;
        if l >= 1 {
            if s[l - 1] == s[l] {
                if dp[r] - dp[l - 1] - 1 >= 0 {
                    println!("{}", dp[r] - dp[l - 1] - 1);
                } else {
                    println!("{}", 0);
                }
            } else {
                if dp[r] - dp[l - 1] >= 0 {
                    println!("{}", dp[r] - dp[l - 1]);
                } else {
                    println!("{}", 0);
                }
            }
        } else {
            println!("{}", dp[r]);
        }
    }
}
