use proconio::{fastout, input};
// use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: String,
        t: String,
    }
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();
    if t[0..n].to_vec() == s {
        if t[(m - n)..m] == s {
            println!("0");
        } else {
            println!("1");
        }
    } else if t[(m - n)..m] == s {
        println!("2");
    } else {
        println!("3");
    }
}
