// use itertools::Itertools;
use proconio::{fastout, input};

fn count_line(l: &Vec<u128>, w: i128) -> u128 {
    let mut width = 0;
    let mut cnt = 1;
    for length in l {
        let length = *length;
        if length > w as u128 {
            return std::i128::MAX as u128;
        }
        if width == 0 {
            width = length;
        } else if width + 1 + length > w as u128 {
            cnt += 1;
            width = length;
        } else {
            if width == 0 {
                width = length
            } else {
                width += 1 + length;
            }
        }
    }
    cnt
}

const INF: i128 = std::i128::MAX / 2;

#[fastout]
fn main() {
    input! {
        n: u128,
        m: u128,
        l: [u128; n]
    }

    let mut left = 0 as i128;
    let mut right = INF;
    while left + 1 < right {
        let mid = (left + right) / 2;
        let needed_lines = count_line(&l, mid);
        if needed_lines > m {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("{}", right);
}
