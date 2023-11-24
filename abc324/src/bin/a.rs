use proconio::{fastout, input};
// use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let b = a[0];
    for a in a {
        if b != a {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
