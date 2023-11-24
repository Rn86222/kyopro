use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n]
    }

    let mut map = HashMap::new();
    for (x, y) in xy.clone() {
        map.insert((x, y), true);
    }

    let mut ans = 0;
    for i in 0..(xy.len() - 1) {
        for j in i..xy.len() {
            let (x1, y1) = xy[i];
            let (x2, y2) = xy[j];
            let (dx, dy) = (x1 - x2, y1 - y2);
            let area = dx * dx + dy * dy;
            if x1 + dy >= 0
                && y1 - dx >= 0
                && x2 + dy >= 0
                && y2 - dx >= 0
                && map.get(&(x1 + dy, y1 - dx)) != None
                && map.get(&(x2 + dy, y2 - dx)) != None
                && ans < area
            {
                ans = area;
            }
            if x1 - dy >= 0
                && y1 + dx >= 0
                && x2 - dy >= 0
                && y2 + dx >= 0
                && map.get(&(x1 - dy, y1 + dx)) != None
                && map.get(&(x2 - dy, y2 + dx)) != None
                && ans < area
            {
                ans = area;
            }
        }
    }
    println!("{}", ans);
}
