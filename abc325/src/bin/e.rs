// use itertools::Itertools;
use proconio::{fastout, input};

use std::{cmp::Reverse, collections::BinaryHeap};

const INF: usize = 1 << 63;

struct Dijkstra {
    distance: Vec<usize>,
    // parent: Vec<usize>,
}

impl Dijkstra {
    // n:usize 頂点の数
    // edge: Vec<Vec<(usize,usize)>> edge[i] = [(2,3), (3,1), (頂点への道,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edge: Vec<Vec<(usize, usize)>>, init: usize) -> Self {
        let mut distance = vec![INF; n];
        let mut parent = vec![INF; n];
        let mut heap = BinaryHeap::new();
        for i in 0..n {
            if i == init {
                // 始点は0
                // BinaryHeapはpopで最大値を取得するので、Reverse()で最小値を取得できるようにする
                heap.push((Reverse(0), i));
            }
            heap.push((Reverse(INF), i));
        }
        while let Some((Reverse(d), target)) = heap.pop() {
            if distance[target] < d {
                // 既にもっと短い経路が見つかってるので無視
                continue;
            }
            distance[target] = d;
            for &(next, cost) in &edge[target] {
                if distance[next] > d + cost {
                    // 短い経路の候補となるので処理を行う
                    distance[next] = d + cost;
                    heap.push((Reverse(distance[next]), next));
                    // ひとつ前の頂点を保存しておく
                    parent[next] = target;
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
        n: usize,
        a: usize,
        b: usize,
        c: usize,
        ds: [[usize; n]; n],
    }
    let mut edges_s = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            edges_s[i].push((j, ds[i][j] * a));
        }
    }
    let mut edges_e = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            edges_e[i].push((j, ds[j][i] * b + c));
        }
    }
    let dijkstra_s = Dijkstra::new(n, edges_s, 0);
    let dijkstra_e = Dijkstra::new(n, edges_e, n - 1);
    let mut ans = std::usize::MAX;
    for i in 0..n {
        if ans > dijkstra_s.distance(i) + dijkstra_e.distance(i) {
            ans = dijkstra_s.distance(i) + dijkstra_e.distance(i);
        }
    }
    println!("{}", ans);
}
