// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n]
    }
    let mut a = a;
    a.sort();
    let mut b = b;
    b.sort();
    let mut c = c;
    c.sort();
    let mut ab_cnt = vec![0; n];
    let mut bc_cnt = vec![0; n];

    for i in 0..n {
        let mut left: i32 = 0;
        let mut right: i32 = n as i32 - 1;
        while left <= right {
            let mid = (left + right) / 2;
            if b[i] > a[mid as usize] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        ab_cnt[i] = left as usize;
    }

    for i in 0..n {
        let mut left: i32 = 0;
        let mut right: i32 = n as i32 - 1;
        while left <= right {
            let mid = (left + right) / 2;
            if b[i] >= c[mid as usize] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        bc_cnt[i] = n - left as usize;
    }

    let mut ans = 0;
    for i in 0..n {
        ans += ab_cnt[i] * bc_cnt[i];
    }

    println!("{}", ans);
}
