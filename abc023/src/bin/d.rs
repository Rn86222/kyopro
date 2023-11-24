// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        hs: [(u128, u128); n]
    }

    let mut hmax = 0;
    let mut smax = 0;
    for (h, s) in hs.clone() {
        if hmax < h {
            hmax = h;
        }
        if smax < s {
            smax = s;
        }
    }
    let mut left = 0;
    let mut right = hmax + smax * n as u128;
    let mut ans = std::u128::MAX;

    while left <= right {
        let mid = (left + right) / 2;
        let mut limit = vec![];
        let mut ok = true;
        for (h, s) in hs.clone() {
            if mid < h {
                ok = false;
                break;
            }
            limit.push((mid - h) / s);
        }
        if !ok {
            left = mid + 1;
            continue;
        }
        limit.sort();
        let mut cnt = 0;
        for l in limit {
            if cnt > l {
                ok = false;
                break;
            }
            cnt += 1;
        }
        if !ok {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    println!("{}", left);
}
