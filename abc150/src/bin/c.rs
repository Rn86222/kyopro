use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n]
    }
    let mut a = 0;
    let mut b = 0;
    for (i, perm) in ((0..n).permutations(n)).enumerate() {
        let perm: Vec<usize> = perm.iter().map(|x| x + 1).collect();
        if perm == p {
            a = i;
        }
        if perm == q {
            b = i;
        }
    }
    println!("{}", if a > b { a - b } else { b - a });
}
