// use itertools::Itertools;
use proconio::{fastout, input};

fn search(n: usize, visited: u32, d: &Vec<Vec<u128>>) -> u128 {
    let mut max = 0;
    if visited.count_ones() == n as u32 {
        return 0;
    }
    for i in 0..n {
        if visited >> i & 1 == 1 {
            continue;
        } else {
            for j in (i + 1)..n {
                if visited >> j & 1 != 1 {
                    let tmp = search(n, visited | 1 << i as u32 | 1 << j as u32, d);
                    if max < tmp + d[i][j - i - 1] {
                        max = tmp + d[i][j - i - 1];
                    }
                }
            }
            break;
        }
    }
    max
}

#[fastout]
fn main() {
    input! {
        n: usize,
        d: [u128; n * (n - 1) / 2]
    }
    let mut new_d = vec![vec![]; n - 1];
    let mut cnt = 0;
    for i in 0..(n - 1) {
        for _ in 0..(n - 1 - i) {
            new_d[i].push(d[cnt]);
            cnt += 1;
        }
        if n % 2 == 1 {
            new_d[i].push(0);
        }
    }
    if n % 2 == 1 {
        new_d.push(vec![0]);
    }
    let d = new_d;
    // println!("{:?}", d);
    let ans = search(if n % 2 == 0 { n } else { n + 1 }, 0, &d);
    println!("{}", ans);
}
