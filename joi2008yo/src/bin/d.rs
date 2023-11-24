use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        m: usize,
        seiza_xy: [(i128, i128); m],
        n: usize,
        shashin_xy: [(i128, i128); n]
    }
    let mut shashin_hash = HashSet::new();
    for (x, y) in shashin_xy.clone() {
        shashin_hash.insert((x, y));
    }
    for (x1, y1) in seiza_xy.clone() {
        for (x2, y2) in shashin_xy.clone() {
            let dx = x2 - x1;
            let dy = y2 - y1;
            let mut ok = true;
            for (x, y) in seiza_xy.clone() {
                if !shashin_hash.contains(&(x + dx, y + dy)) {
                    ok = false;
                }
            }
            if ok {
                println!("{} {}", dx, dy);
                return;
            }
        }
    }
}
