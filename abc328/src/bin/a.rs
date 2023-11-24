use proconio::{fastout, input};
// use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        n:usize,
        x: usize,
        ss: [usize; n],
    }

    let mut sum = 0;
    for s in ss {
        if s <= x {
            sum += s;
        }
    }

    println!("{}", sum);
}
