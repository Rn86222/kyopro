// use itertools::Itertools;
use proconio::{fastout, input};
use std::{cmp::Reverse, collections::BinaryHeap};

#[fastout]
fn main() {
    input! {
        n: usize,
        tds: [(u128, u128); n]
    }
    let mut tds = tds;
    tds.sort_by(|a, b| a.0.cmp(&b.0));
    let exit_ts = tds.iter().map(|td| td.0 + td.1).collect::<Vec<_>>();
    let mut heap = BinaryHeap::new();
    let mut tds_cnt = 0;
    let mut current_time = 1;
    let mut ans = 0;
    loop {
        while tds_cnt < n && tds[tds_cnt].0 <= current_time {
            heap.push(Reverse(exit_ts[tds_cnt]));
            tds_cnt += 1;
        }
        if let Some(Reverse(t)) = heap.pop() {
            if t < current_time {
                continue;
            }
            ans += 1;
            current_time += 1;
        } else {
            if tds_cnt == n {
                break;
            }
            current_time = tds[tds_cnt].0;
        }
    }
    println!("{}", ans);
}
