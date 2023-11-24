// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }
    let mut a = a;
    a.sort();
    if n == 1 {
        println!("1");
        return;
    }
    let mut end = 1;
    let mut ans = 0;

    for i in 0..n {
        if end <= i {
            end = i + 1;
        }
        let end_pos = a[i] + m;
        while end < n && a[end] < end_pos {
            end += 1;
        }
        if ans < end - i {
            ans = end - i;
        }
    }
    println!("{}", ans);
}
