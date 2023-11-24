use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(usize, usize); m]
    }
    let mut xy_hash = HashSet::new();
    for (x, y) in xy {
        xy_hash.insert((x - 1, y - 1));
        xy_hash.insert((y - 1, x - 1));
    }
    let bit = 2_u32.pow(n as u32);
    let mut ans = 0;
    for i in 0..bit {
        let mut group = vec![];
        for j in 0..n {
            if i >> j & 1 == 1 {
                group.push(j);
            }
        }
        if ans >= group.len() {
            continue;
        }
        let mut ok = true;
        'outer: for j in 0..(group.len() - 1) {
            for k in (j + 1)..group.len() {
                if !xy_hash.contains(&(group[j as usize], group[k as usize])) {
                    ok = false;
                    break 'outer;
                }
            }
        }
        if ok {
            ans = group.len();
        }
    }
    println!("{}", ans);
}
