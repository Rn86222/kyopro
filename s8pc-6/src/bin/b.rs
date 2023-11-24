use proconio::{fastout, input};

fn min(ab: &Vec<(i128, i128)>, s: i128, t: i128) -> i128 {
    let mut tmp = 0;
    for (a, b) in ab.clone() {
        tmp += (a - b).abs();
        tmp += if (s - a).abs() + (t - b).abs() < (s - b).abs() + (t - a).abs() {
            (s - a).abs() + (t - b).abs()
        } else {
            (s - b).abs() + (t - a).abs()
        };
    }
    tmp
}

#[fastout]
fn main() {
    input! {
        n: usize,
        ab:[(i128, i128); n]
    }
    let mut ans = std::i128::MAX;
    for (a1, b1) in ab.clone() {
        for (a2, b2) in ab.clone() {
            let s = a1;
            let t = b1;
            let tmp = min(&ab, s, t);
            if ans > tmp {
                ans = tmp;
            }
            let s = a2;
            let t = b1;
            let tmp = min(&ab, s, t);
            if ans > tmp {
                ans = tmp;
            }
            let s = a1;
            let t = b2;
            let tmp = min(&ab, s, t);
            if ans > tmp {
                ans = tmp;
            }
            let s = a2;
            let t = b2;
            let tmp = min(&ab, s, t);
            if ans > tmp {
                ans = tmp;
            }
        }
    }
    println!("{}", ans);
}
