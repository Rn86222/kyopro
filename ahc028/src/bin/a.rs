use std::collections::HashMap;

use itertools::Itertools;
use proconio::{fastout, input};

type KeyBoard = Vec<Vec<char>>;
type LuckyWord = Vec<char>;
type KeyMap = HashMap<char, Vec<(usize, usize)>>;

fn init(a: &Vec<String>, ts: &Vec<String>) -> (KeyBoard, Vec<LuckyWord>, KeyMap) {
    let mut tmp = vec![];
    for line in a {
        tmp.push(line.chars().collect_vec());
    }
    let a = tmp;
    let mut tmp = vec![];
    for line in ts {
        tmp.push(line.chars().collect_vec());
    }
    let ts = tmp;

    let mut key_map = HashMap::new();
    for (i, line) in a.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            key_map.entry(*c).or_insert(vec![]).push((i, j));
        }
    }
    (a, ts, key_map)
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s_i: usize,
        s_j: usize,
        a: [String; n],
        ts: [String; m],
    }

    let (a, ts, key_map) = init(&a, &ts);
    let mut pos_i = s_i;
    let mut pos_j = s_j;

    let mut unvisited = vec![];
    for t in &ts {
        unvisited.push(t.clone());
    }

    let sampling_num = m;
    let mut s = vec![];

    while !unvisited.is_empty() {
        let mut min_cost = isize::MAX;
        let mut min_route = vec![];
        let mut argmin = 0;
        for (t_idx, t) in unvisited.iter().take(sampling_num).enumerate() {
            let mut cost = 0;
            let mut route = vec![];
            let mut overlap = 0;

            let mut tmp_pos_i = pos_i;
            let mut tmp_pos_j = pos_j;

            for i in 0..t.len() {
                if s.len() + i >= t.len() && s[s.len() - t.len() + i..] == t[..t.len() - i] {
                    overlap = t.len() - i;
                    break;
                }
            }

            for (c_idx, c) in t.iter().enumerate() {
                if c_idx < overlap {
                    continue;
                }
                let mut min_dist = isize::MAX;
                let mut min_i = 0;
                let mut min_j = 0;
                for (i, j) in key_map.get(c).unwrap() {
                    let dist = (tmp_pos_i as isize - *i as isize).abs()
                        + (tmp_pos_j as isize - *j as isize).abs();
                    if dist < min_dist {
                        min_dist = dist;
                        min_i = *i;
                        min_j = *j;
                    }
                }
                tmp_pos_i = min_i;
                tmp_pos_j = min_j;
                route.push((min_i, min_j));
                cost += min_dist;
            }
            if cost < min_cost {
                min_cost = cost;
                min_route = route;
                argmin = t_idx;
            }
        }
        for r in &min_route {
            s.push(a[r.0][r.1]);
            println!("{} {}", r.0, r.1);
        }
        pos_i = min_route.last().unwrap().0;
        pos_j = min_route.last().unwrap().1;
        unvisited.remove(argmin);
    }
}
