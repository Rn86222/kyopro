use std::collections::HashSet;

// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        cs: [usize; n],
        queries: [(usize, usize); q],
    }
    let mut sets = vec![HashSet::new(); n];
    let mut index_map = (0..n).collect::<Vec<_>>();
    for i in 0..n {
        sets[i].insert(cs[i]);
    }
    for (a, b) in queries {
        let a = a - 1;
        let b = b - 1;
        let a_size = sets[index_map[a]].len();
        let b_size = sets[index_map[b]].len();
        if a_size > b_size {
            let tmp = index_map[a];
            index_map[a] = index_map[b];
            index_map[b] = tmp;
        }
        for e in sets[index_map[a]].clone() {
            sets[index_map[b]].insert(e);
        }
        sets[index_map[a]].clear();
        println!("{}", sets[index_map[b]].len());
    }
}
