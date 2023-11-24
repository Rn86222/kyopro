mod unionfind;
use proconio::input;
use unionfind::*;

fn main() {
    input! {
        N: usize,
        M: usize,
        p: [usize; N],
        xy: [(usize, usize); M]
    }

    let mut uf = UnionFind::new(N);

    for (x, y) in xy {
        uf.unite(x - 1, y - 1);
    }

    let mut ans = 0;
    for i in 0..N {
        if uf.same(i, p[i] - 1) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
