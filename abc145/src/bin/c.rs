use itertools::Itertools;
use proconio::{fastout, input};

fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
    (((x1 - x2) * (x1 - x2)) as f64 + ((y1 - y2) * (y1 - y2)) as f64).sqrt()
}

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n]
    }
    let mut sum = 0.;
    let permutations: Vec<Vec<usize>> = (0..n).permutations(n).collect();
    for perm in permutations.clone() {
        let mut dis = 0.;
        for i in 1..perm.len() {
            let (x1, y1) = xy[perm[i - 1]];
            let (x2, y2) = xy[perm[i]];
            dis += distance(x1, y1, x2, y2);
        }
        sum += dis;
    }
    println!("{}", sum / permutations.len() as f64);
}
