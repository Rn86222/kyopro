use proconio::{fastout, input};
// use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: String
    }
    let s: Vec<char> = s.chars().collect();
    for i in 0..(n - 2) {
        let seg = s[i..=(i + 2)].to_vec();
        if seg == vec!['A', 'B', 'C'] {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
