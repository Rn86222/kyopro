// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m]
    }
    let mut max = 0;
    let mut max_id = 0;
    let mut counts = vec![0; n + 1];
    for i in 0..m {
        counts[a[i]] += 1;
        if max < counts[a[i]] {
            max = counts[a[i]];
            max_id = a[i];
        } else if max == counts[a[i]] && max_id > a[i] {
            max_id = a[i];
        }
        println!("{}", max_id);
    }
}
