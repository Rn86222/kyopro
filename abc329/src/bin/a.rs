use proconio::{fastout, input};
// use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        s: String
    }
    let s = s.chars().collect::<Vec<char>>();
    for i in 0..s.len() {
        print!("{} ", s[i]);
    }
    println!();
}
