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
