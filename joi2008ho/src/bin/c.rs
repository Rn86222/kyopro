use proconio::{fastout, input};

fn upper_bound(vec: &Vec<u128>, b: u128) -> usize {
    let mut left = 0;
    let mut right = vec.len() - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if vec[mid] <= b {
            left = mid + 1;
        } else {
            if mid == 0 {
                return 0;
            }
            right = mid - 1;
        }
    }
    left
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: u128,
        p: [u128; n]
    }

    let mut ans1 = 0;
    for i in 0..n {
        if p[i] <= m && ans1 < p[i] {
            ans1 = p[i];
        }
    }
    let mut ans2 = 0;
    let mut pattern2 = vec![];
    for i in 0..n {
        for j in i..n {
            if p[i] + p[j] <= m {
                if ans2 < p[i] + p[j] {
                    ans2 = p[i] + p[j];
                }
                pattern2.push(p[i] + p[j]);
            }
        }
    }
    pattern2.sort();
    let mut ans3 = 0;
    for i in 0..n {
        if p[i] > m {
            continue;
        }
        let index = upper_bound(&pattern2, m - p[i]);
        if index == 0 {
            continue;
        }
        if ans3 < p[i] + pattern2[index - 1] {
            ans3 = p[i] + pattern2[index - 1];
        }
    }
    let mut ans4 = 0;
    for i in 0..pattern2.len() {
        let index = upper_bound(&pattern2, m - pattern2[i]);
        if index == 0 {
            continue;
        }
        if ans4 < pattern2[i] + pattern2[index - 1] {
            ans4 = pattern2[i] + pattern2[index - 1];
        }
    }
    let ans12 = if ans1 > ans2 { ans1 } else { ans2 };
    let ans34 = if ans3 > ans4 { ans3 } else { ans4 };

    println!("{}", if ans12 > ans34 { ans12 } else { ans34 });
}
