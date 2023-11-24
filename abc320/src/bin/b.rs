use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    }
    let s: Vec<char> = s.chars().collect();
    let mut ans = 0;
    for i in 0..(s.len() - 1) {
        for j in i..(s.len() + 1) {
            let seg = s[i..j].to_vec();
            let mut rev = seg.clone();
            rev.reverse();
            if seg == rev {
                if j - i > ans {
                    ans = j - i;
                }
            }
        }
    }
    println!("{}", ans);
}
