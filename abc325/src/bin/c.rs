// use itertools::Itertools;
use proconio::{fastout, input};
use std::collections::{HashMap, HashSet};

fn dfs(
    graph: &HashMap<i32, Vec<i32>>,
    seen: &mut HashSet<i32>,
    h: usize,
    w: usize,
    i: i32,
    j: i32,
) {
    if i < 0 || i >= h as i32 || j < 0 || j >= w as i32 {
        return;
    }
    seen.insert(i * w as i32 + j);
    let edges = graph.get(&(i * w as i32 + j));
    if edges.is_none() {
        return;
    }
    for edge in edges.unwrap() {
        if !seen.contains(edge) {
            dfs(graph, seen, h, w, edge / w as i32, edge % w as i32);
        }
    }
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        ss: [String; h],
    }
    let mut tmp_ss: Vec<Vec<char>> = vec![];
    for s in ss {
        tmp_ss.push(s.chars().collect());
    }
    let dxs = [-1, 0, 1];
    let dys = [-1, 0, 1];
    let ss = tmp_ss;
    let mut graph = HashMap::<i32, Vec<i32>>::new();
    for i in 0..h {
        for j in 0..w {
            if ss[i][j] == '#' {
                for dx in dxs.clone() {
                    for dy in dys.clone() {
                        if (dx != 0 || dy != 0)
                            && i as i32 + dy < h as i32
                            && i as i32 + dy >= 0 as i32
                            && j as i32 + dx < w as i32
                            && j as i32 + dx >= 0 as i32
                            && ss[(i as i32 + dy) as usize][(j as i32 + dx) as usize] == '#'
                        {
                            let i = i as i32;
                            let j = j as i32;
                            let w = w as i32;
                            let edges = graph.get(&(i * w + j));
                            if edges.is_none() {
                                graph.insert(i * w + j, vec![(i + dy) * w + (j + dx)]);
                            } else {
                                let mut edges = edges.unwrap().clone();
                                edges.push((i + dy) * w + (j + dx));
                                graph.insert(i * w + j, edges);
                            }
                            // let edges = graph.get(&((i + dy) * w + (j + dx)));
                            // if edges.is_none() {
                            //     graph.insert((i + dy) * w + (j + dx), vec![i * w + j]);
                            // } else {
                            //     let mut edges = edges.unwrap().clone();
                            //     edges.push(i * w + j);
                            //     graph.insert((i + dy) * w + (j + dx), edges);
                            // }
                        }
                    }
                }
            }
        }
    }
    let seen = &mut HashSet::<i32>::new();
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if !seen.contains(&((i * w + j) as i32)) && ss[i][j] == '#' {
                dfs(&graph, seen, h, w, i as i32, j as i32);
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
