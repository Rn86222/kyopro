use itertools::Itertools;
use proconio::{fastout, input};

fn dfs(
    coord: &mut Vec<(i128, i128)>,
    decided: &mut Vec<bool>,
    start: usize,
    edges: &Vec<Vec<(usize, (i128, i128))>>,
) {
    if decided[start] {
        return;
    }
    decided[start] = true;
    for (t, (x, y)) in edges[start].clone() {
        let (sx, sy) = coord[start];
        coord[t] = (sx + x, sy + y);
        dfs(coord, decided, t, edges);
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abxy: [(usize, usize, i128, i128); m]
    }
    let mut decided = vec![false; n + 1];
    let mut coord = vec![(0, 0); n + 1];
    let mut edges = vec![vec![]; n + 1];
    for (a, b, x, y) in abxy.clone() {
        edges[a].push((b, (x, y)));
        edges[b].push((a, (-x, -y)));
    }
    dfs(&mut coord, &mut decided, 1, &edges);
    for i in 1..=n {
        if decided[i] {
            let (x, y) = coord[i];
            println!("{} {}", x, y);
        } else {
            println!("undecidable");
        }
    }
}
