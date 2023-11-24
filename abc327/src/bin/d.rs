// use itertools::Itertools;
use proconio::{fastout, input};

pub struct UnionFind {
    par: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let par = (0..n).collect();
        UnionFind { par }
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            self.par[x] = self.root(self.par[x]);
            self.par[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let rx = self.root(x);
        let ry = self.root(y);
        if rx != ry {
            self.par[rx] = ry;
        }
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        return self.root(x) == self.root(y);
    }
}

pub fn is_bigraph(neigh: &[Vec<usize>]) -> bool {
    let n = neigh.len();
    let mut uf = UnionFind::new(n * 2);
    for u in 0..n {
        for &v in neigh[u].iter() {
            uf.unite(u * 2, v * 2 + 1);
            uf.unite(u * 2 + 1, v * 2);
        }
    }
    for u in 0..n {
        if uf.same(u * 2, u * 2 + 1) {
            return false;
        }
    }
    true
}

// fn dfs(v: usize, color: i32, colors: &mut Vec<i32>, graph: &Vec<Vec<usize>>) -> bool {
//     colors[v] = color;
//     for &u in &graph[v] {
//         if colors[u] == color {
//             return false;
//         }
//         if colors[u] == 0 && !dfs(u, -color, colors, graph) {
//             return false;
//         }
//     }
//     true
// }

// fn is_bigraph(graph: &Vec<Vec<usize>>) -> bool {
//     let n = graph.len();
//     let mut colors = vec![0; n];
//     return dfs(0, 1, &mut colors, graph);
// }

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
        b: [usize; m],
    }

    let mut graph = vec![vec![]; n];
    for i in 0..m {
        graph[a[i] - 1].push(b[i] - 1);
        graph[b[i] - 1].push(a[i] - 1);
    }
    if is_bigraph(&graph) {
        println!("Yes");
    } else {
        println!("No");
    }
}
