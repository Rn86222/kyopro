// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize
    }
    for i in n..=1000 {
        let msb = (i / 100) % 10;
        let middle = (i / 10) % 10;
        let lsb = i % 10;
        if msb * middle == lsb {
            println!("{}", i);
            return;
        }
    }
}
