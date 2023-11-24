use proconio::{fastout, input};
// use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        s: String
    }

    let s: Vec<char> = s.chars().collect();
    for i in 1..(s.len() / 2 + 1) {
        if s[2 * i - 1] != '0' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
