use itertools::Itertools;
use proconio::{fastout, input};

/// 指定された要素以上の値が現れる最小のインデックス<br>条件を満たす値がない場合は 0 を返す
fn lower_bound<T: PartialOrd>(vec: &Vec<T>, bound: T) -> usize {
    let mut left: i64 = 0;
    let mut right: i64 = vec.len() as i64 - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if vec[mid as usize] < bound {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    left as usize
}

/// 指定された要素より大きい値が現れる最小のインデックス<br>条件を満たす値がない場合は vec.len() を返す
fn upper_bound<T: PartialOrd>(vec: &Vec<T>, bound: T) -> usize {
    let mut left: i64 = 0;
    let mut right: i64 = vec.len() as i64 - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if vec[mid as usize] <= bound {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    left as usize
}

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        p: u128,
        mut f: [u128; n]
    }

    f.sort();
    let shuyu_cost = p / d as u128;
    let len = f.len();
    let first = upper_bound(&f, shuyu_cost);
    let shuyu_num = (len - first) / d;
    let mut cost = 0;
    if len >= (shuyu_num + 1) * d {
        for i in 0..(len - (shuyu_num + 1) * d) {
            cost += f[i];
        }
    }
    cost += p * shuyu_num as u128;
    let mut tmp = 0;
    if len >= (shuyu_num + 1) * d {
        for i in (len - (shuyu_num + 1) * d)..(len - shuyu_num * d) {
            tmp += f[i];
        }
    } else {
        for i in 0..(len - shuyu_num * d) {
            tmp += f[i];
        }
    }
    if p > tmp {
        cost += tmp;
    } else {
        cost += p;
    }
    println!("{}", cost);
}
