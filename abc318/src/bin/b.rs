use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n]
    }
    let mut map = vec![vec![false; 100]; 100];
    for (a, b, c, d) in abcd {
        for i in c..d {
            for j in a..b {
                map[i][j] = true;
            }
        }
    }
    let mut s = 0;
    for i in 0..100 {
        for j in 0..100 {
            if map[i][j] {
                s += 1;
            }
        }
    }
    println!("{}", s);
}
