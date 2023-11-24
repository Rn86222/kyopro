use proconio::{fastout, input};
// use itertools::Itertools;
use std::collections::HashMap;

// use std::cmp::Ordering;

// fn find_max_n(m: u128) -> u32 {
//     let n = (f64::log2(m as f64)).ceil() as u32;
//     n
// }

#[fastout]
fn main() {
    input! {
        n: usize,
        sc: [(u128, u128); n]
    }
    let mut sc_map = HashMap::new();

    let mut marked = vec![false; n];
    let mut ans = 0;
    for (_, c) in sc.clone() {
        ans += c;
    }
    let mut sc = sc.clone();
    sc.sort_by(|a, b| a.0.cmp(&b.0));
    for (i, (s, c)) in sc.iter().enumerate() {
        sc_map.insert(s, (i, *c));
    }
    for i in 0..n {
        if marked[i] {
            continue;
        }
        // println!("{} {}", i, sc[i].0);
        ans -= sc[i].1 / 2;
        marked[i] = true;
        // let mut current_index = i;
        let mut current_size = sc[i].0 * 2;
        let mut new_num = sc[i].1 / 2;
        if new_num == 0 {
            continue;
        }
        loop {
            let next = sc_map.get(&current_size);
            match next {
                None => {
                    let next_num = new_num;
                    ans -= next_num / 2;
                    new_num = next_num / 2;
                    current_size *= 2;
                }
                Some((index, next_num)) => {
                    let next_num = *next_num + new_num;
                    ans -= next_num / 2;
                    marked[*index] = true;
                    // current_index = *index;
                    new_num = next_num / 2;
                    current_size *= 2;
                }
            }
            if new_num == 0 {
                break;
            }
        }
    }
    println!("{}", ans);
}
