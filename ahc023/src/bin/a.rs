use proconio::{fastout, input};
use std::time::Duration;
use std::time::Instant;

use std::{cmp::Reverse, collections::BinaryHeap};

const INF: usize = 1 << 31;

struct Dijkstra {
    pub distance: Vec<usize>,
}

impl Dijkstra {
    pub fn new(n: usize, edge: Vec<Vec<usize>>, init: usize) -> Self {
        let mut distance = vec![INF; n];
        let mut heap = BinaryHeap::new();
        for i in 0..n {
            if i == init {
                heap.push((Reverse(0), i));
            }
            heap.push((Reverse(INF), i));
        }
        while let Some((Reverse(d), target)) = heap.pop() {
            if distance[target] < d {
                continue;
            }
            distance[target] = d;
            for &next in &edge[target] {
                if distance[next] > d + 1 {
                    distance[next] = d + 1;
                    heap.push((Reverse(distance[next]), next));
                }
            }
        }
        Self { distance }
    }

    pub fn distance(&self, target: usize) -> usize {
        self.distance[target]
    }
}

#[fastout]
fn main() {
    input! {
        _: usize,
        height: usize,
        width: usize,
        i0: usize,
        hs: [String; height - 1],
        vs: [String; width],
        k: usize,
        sd: [(usize, usize); k]
    }
    let start_time = Instant::now();
    let mut h = vec![];
    for line in hs {
        let line: Vec<char> = line.chars().collect();
        h.push(line);
    }
    let mut v = vec![];
    for line in vs {
        let line: Vec<char> = line.chars().collect();
        v.push(line);
    }

    let mut edges = vec![];
    for i in 0..height {
        for j in 0..width {
            let mut edge = vec![];
            if i >= 1 && h[i - 1][j] == '0' {
                edge.push((i - 1) * width + j);
            }
            if i <= height - 2 && h[i][j] == '0' {
                edge.push((i + 1) * width + j);
            }
            if j >= 1 && v[i][j - 1] == '0' {
                edge.push(i * width + (j - 1));
            }
            if j <= width - 2 && v[i][j] == '0' {
                edge.push(i * width + (j + 1));
            }
            edges.push(edge);
        }
    }

    let dijkstra = Dijkstra::new(height * width, edges, i0 * width);

    let mut distance = vec![];
    for i in 0..(height * width) {
        distance.push((dijkstra.distance(i), (i / width, i % width)));
    }
    distance.sort_by(|(a, _), (b, _)| b.cmp(a));

    let mut score: Vec<(usize, (usize, usize))> = sd
        .iter()
        .enumerate()
        .map(|(i, (s, d))| (i, (*s, *d)))
        .collect();
    score.sort_by(|(_, (_, d1)), (_, (_, d2))| d2.cmp(d1));

    let mut sorted_scores: Vec<usize> = sd.iter().map(|(s, d)| d - s).collect();
    sorted_scores.sort_by(|a, b| b.cmp(a));
    let lower_score;
    if k < height * width {
        lower_score = 0;
    } else {
        lower_score = sorted_scores[height * width - 1];
    }

    let mut ans = vec![];
    let mut cnt = 0;

    for (index, (s, d)) in score {
        if d - s < lower_score {
            continue;
        }
        if start_time.elapsed() >= Duration::from_millis(1900) {
            break;
        }
        if cnt >= height * width {
            break;
        }
        let (_, (i, j)) = distance[cnt];
        ans.push((index + 1, i, j, 1, d));
        cnt += 1;
    }
    println!("{}", ans.len());
    for (k, i, j, s, _) in ans {
        println!("{} {} {} {}", k, i, j, s);
    }
}
