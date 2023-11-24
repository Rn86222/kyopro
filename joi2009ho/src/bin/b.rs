// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: i128,
        n: usize,
        m: usize,
        mut d: [i128; n - 1],
        k: [i128; m]
    }
    d.sort();
    let mut first = vec![0];
    first.append(&mut d);
    first.append(&mut vec![l]);
    let d = first;
    let mut ans = 0;

    for dist in k {
        let mut left: usize = 0;
        let mut right: usize = n;
        let mut min = std::i128::MAX;
        while left <= right {
            let mid = (left + right) / 2;
            if (dist - d[mid]).abs() < min {
                min = (dist - d[mid]).abs();
            }
            if dist < d[mid] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        ans += min;
    }

    println!("{}", ans);
}
