// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        ps: [usize; n]
    }
    if n > 1 {
        let mut start_index = 0;
        loop {
            if start_index + k > n {
                break;
            }
            let mut before = ps[start_index];
            let mut found = false;
            for i in (start_index + 1)..(start_index + k) {
                if before > ps[i] {
                    found = true;
                    start_index = i;
                    break;
                } else {
                    before = ps[i];
                }
            }
            if !found {
                for p in ps.clone() {
                    print!("{} ", p);
                }
                println!("");
                return;
            }
        }
    }
    if n > k {
        let mut before = ps[n - k - 1];
        let mut count = 1;
        for i in (0..(n - k - 1)).rev() {
            if ps[i] > before {
                break;
            } else {
                before = ps[i];
                count += 1;
            }
        }
        let min = ps[n - k - 1];
        let mut ok = true;
        for i in (n - k)..(n - count) {
            if min > ps[i] {
                ok = false;
            }
        }
        if !ok {
            count = 0;
        }
        // let mut min = std::u64::MAX as usize;
        // for i in (n - k)..n {
        //     if min > ps[i] {
        //         min = ps[i];
        //     }
        // }
        // let mut before = min;
        // let mut count = 0;
        // for i in (0..(n - k)).rev() {
        //     if ps[i] > before {
        //         break;
        //     } else {
        //         before = ps[i];
        //         count += 1;
        //     }
        // }
        let mut k_seg = ps[(n - k - count)..(n - count)].to_vec();
        k_seg.sort();
        for i in 0..(n - k - count) {
            print!("{} ", ps[i]);
        }
        for i in 0..k {
            print!("{} ", k_seg[i]);
        }
        for i in (n - count)..n {
            print!("{} ", ps[i]);
        }
        println!("");
    } else {
        let mut sorted_ps = ps.clone();
        sorted_ps.sort();
        for p in sorted_ps {
            print!("{} ", p);
        }
        println!("");
    }
}
