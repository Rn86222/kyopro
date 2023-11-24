pub struct SegTree<X: Clone> {
    n: usize,
    fx: fn(X, X) -> X,
    ex: X,
    pub dat: Vec<X>,
}

impl<X: Clone> SegTree<X> {
    pub fn new(n_: usize, fx: fn(X, X) -> X, ex: X) -> Self {
        let mut x = 1;
        while n_ > x {
            x *= 2;
        }
        let n = x;
        let dat = vec![ex.clone(); n * 4];
        SegTree { n, fx, ex, dat }
    }

    pub fn set(&mut self, i: usize, v: X) {
        self.dat[i + self.n - 1] = v;
    }

    pub fn build(&mut self) {
        for k in (0..(self.n - 1)).rev() {
            self.dat[k] = (self.fx)(self.dat[2 * k + 1].clone(), self.dat[2 * k + 2].clone());
        }
    }

    pub fn update(&mut self, i: usize, v: X) {
        let mut i = i + self.n - 1;
        self.dat[i] = v;
        while i > 0 {
            i = (i - 1) / 2;
            self.dat[i] = (self.fx)(self.dat[2 * i + 1].clone(), self.dat[2 * i + 2].clone());
        }
    }

    fn query_sub(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> X {
        if r <= a || b <= l {
            self.ex.clone()
        } else if a <= l && r <= b {
            self.dat[k].clone()
        } else {
            let vl = self.query_sub(a, b, 2 * k + 1, l, (l + r) / 2);
            let vr = self.query_sub(a, b, 2 * k + 2, (l + r) / 2, r);
            (self.fx)(vl, vr)
        }
    }

    pub fn query(&self, a: usize, b: usize) -> X {
        self.query_sub(a, b, 0, 0, self.n)
    }
}

impl<X: Clone + PartialOrd> SegTree<X> {
    fn find_rightest_sub(&self, a: usize, b: usize, v: X, k: usize, l: usize, r: usize) -> i64 {
        if self.dat[k] > v || r <= a || b <= l {
            (a - 1) as i64
        } else if k >= self.n - 1 {
            (k - (self.n - 1)) as i64
        } else {
            let vr = self.find_rightest_sub(a, b, v.clone(), 2 * k + 2, (l + r) / 2, r);
            if vr != (a - 1) as i64 {
                vr
            } else {
                self.find_rightest_sub(a, b, v, 2 * k + 1, l, (l + r) / 2)
            }
        }
    }

    fn find_leftest_sub(&self, a: usize, b: usize, v: X, k: usize, l: usize, r: usize) -> i64 {
        if self.dat[k] > v || r <= a || b <= l {
            b as i64
        } else if k >= self.n - 1 {
            (k - (self.n - 1)) as i64
        } else {
            let vl = self.find_rightest_sub(a, b, v.clone(), 2 * k + 1, l, (l + r) / 2);
            if vl != b as i64 {
                vl
            } else {
                self.find_rightest_sub(a, b, v, 2 * k + 2, (l + r) / 2, r)
            }
        }
    }

    pub fn find_rightest(&self, a: usize, b: usize, v: X) -> usize {
        self.find_rightest_sub(a, b, v, 0, 0, self.n) as usize
    }

    pub fn find_leftest(&self, a: usize, b: usize, v: X) -> usize {
        self.find_leftest_sub(a, b, v, 0, 0, self.n) as usize
    }
}

pub struct SegTreeLazy<X: Clone, M: Clone> {
    n: usize,
    fx: fn(X, X) -> X,
    fa: fn(X, M) -> X,
    fm: fn(M, M) -> M,
    ex: X,
    em: M,
    pub dat: Vec<X>,
    pub lazy: Vec<M>,
}

impl<X: Clone, M: Clone + PartialEq> SegTreeLazy<X, M> {
    pub fn new(
        n_: usize,
        fx: fn(X, X) -> X,
        fa: fn(X, M) -> X,
        fm: fn(M, M) -> M,
        ex: X,
        em: M,
    ) -> Self {
        let mut x = 1;
        while n_ > x {
            x *= 2;
        }
        let n = x;
        let dat = vec![ex.clone(); n * 4];
        let lazy = vec![em.clone(); n * 4];
        SegTreeLazy {
            n,
            fx,
            fa,
            fm,
            ex,
            em,
            dat,
            lazy,
        }
    }

    pub fn set(&mut self, i: usize, v: X) {
        self.dat[i + self.n - 1] = v;
    }

    pub fn build(&mut self) {
        for k in (0..(self.n - 1)).rev() {
            self.dat[k] = (self.fx)(self.dat[2 * k + 1].clone(), self.dat[2 * k + 2].clone());
        }
    }

    pub fn eval(&mut self, k: usize) {
        if self.lazy[k] == self.em {
            return;
        }
        if k < self.n - 1 {
            self.lazy[2 * k + 1] = (self.fm)(self.lazy[2 * k + 1].clone(), self.lazy[k].clone());
            self.lazy[2 * k + 2] = (self.fm)(self.lazy[2 * k + 2].clone(), self.lazy[k].clone());
        }
        self.dat[k] = (self.fa)(self.dat[k].clone(), self.lazy[k].clone());
        self.lazy[k] = self.em.clone();
    }

    fn update_sub(&mut self, a: usize, b: usize, v: M, k: usize, l: usize, r: usize) {
        self.eval(k);
        if a <= l && r <= b {
            self.lazy[k] = (self.fm)(self.lazy[k].clone(), v);
            self.eval(k);
        } else if a < r && l < b {
            self.update_sub(a, b, v.clone(), 2 * k + 1, l, (l + r) / 2);
            self.update_sub(a, b, v, 2 * k + 2, (l + r) / 2, r);
            self.dat[k] = (self.fx)(self.dat[2 * k + 1].clone(), self.dat[2 * k + 2].clone());
        }
    }

    pub fn update(&mut self, a: usize, b: usize, v: M) {
        self.update_sub(a, b, v, 0, 0, self.n);
    }

    fn query_sub(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> X {
        if r <= a || b <= l {
            self.ex.clone()
        } else if a <= l && r <= b {
            self.dat[k].clone()
        } else {
            let vl = self.query_sub(a, b, 2 * k + 1, l, (l + r) / 2);
            let vr = self.query_sub(a, b, 2 * k + 2, (l + r) / 2, r);
            (self.fx)(vl, vr)
        }
    }

    pub fn query(&self, a: usize, b: usize) -> X {
        self.query_sub(a, b, 0, 0, self.n)
    }
}

impl<X: Clone + PartialOrd, M: Clone + PartialEq> SegTreeLazy<X, M> {
    fn find_rightest_sub(&mut self, a: usize, b: usize, v: X, k: usize, l: usize, r: usize) -> i64 {
        self.eval(k);
        if self.dat[k] > v || r <= a || b <= l {
            (a - 1) as i64
        } else if k >= self.n - 1 {
            (k - (self.n - 1)) as i64
        } else {
            let vr = self.find_rightest_sub(a, b, v.clone(), 2 * k + 2, (l + r) / 2, r);
            if vr != (a - 1) as i64 {
                vr
            } else {
                self.find_rightest_sub(a, b, v, 2 * k + 1, l, (l + r) / 2)
            }
        }
    }

    fn find_leftest_sub(&mut self, a: usize, b: usize, v: X, k: usize, l: usize, r: usize) -> i64 {
        self.eval(k);
        if self.dat[k] > v || r <= a || b <= l {
            b as i64
        } else if k >= self.n - 1 {
            (k - (self.n - 1)) as i64
        } else {
            let vl = self.find_rightest_sub(a, b, v.clone(), 2 * k + 1, l, (l + r) / 2);
            if vl != b as i64 {
                vl
            } else {
                self.find_rightest_sub(a, b, v, 2 * k + 2, (l + r) / 2, r)
            }
        }
    }

    pub fn find_rightest(&mut self, a: usize, b: usize, v: X) -> usize {
        self.find_rightest_sub(a, b, v, 0, 0, self.n) as usize
    }

    pub fn find_leftest(&mut self, a: usize, b: usize, v: X) -> usize {
        self.find_leftest_sub(a, b, v, 0, 0, self.n) as usize
    }
}

pub struct SegTreeDual<X: Clone, M: Clone> {
    n: usize,
    fa: fn(X, M) -> X,
    fm: fn(M, M) -> M,
    em: M,
    dat: Vec<X>,
    lazy: Vec<M>,
}

impl<X: Clone, M: Clone + PartialEq> SegTreeDual<X, M> {
    pub fn new(n_: usize, fa: fn(X, M) -> X, fm: fn(M, M) -> M, em: M, initial_value: X) -> Self {
        let mut x = 1;
        while n_ > x {
            x *= 2;
        }
        let n = x;
        let dat = vec![initial_value.clone(); n];
        let lazy = vec![em.clone(); n - 1];
        SegTreeDual {
            n,
            fa,
            fm,
            em,
            dat,
            lazy,
        }
    }

    pub fn point_get(&self, i: usize) -> X {
        let mut acc = self.dat[i].clone();
        let mut j = (i + self.n) / 2;
        while j > 0 {
            acc = (self.fa)(acc, self.lazy[i - 1].clone());
            j /= 2;
        }
        acc
    }

    fn range_apply_sub(&mut self, i: usize, a: usize, b: usize, l: usize, r: usize, v: M) {
        if a <= l && r <= b {
            if i < self.n - 1 {
                self.lazy[i] = (self.fm)(v, self.lazy[i].clone());
            } else {
                self.dat[i - self.n + 1] = (self.fa)(self.dat[i - self.n + 1].clone(), v);
            }
        } else if a < r && l < b {
            self.range_apply_sub(2 * i + 1, 0, self.n, l, (l + r) / 2, self.lazy[i].clone());
            self.range_apply_sub(2 * i + 2, 0, self.n, (l + r) / 2, r, self.lazy[i].clone());
            self.lazy[i] = self.em.clone();
            self.range_apply_sub(2 * i + 1, a, b, l, (l + r) / 2, v.clone());
            self.range_apply_sub(2 * i + 2, a, b, (l + r) / 2, r, v);
        }
    }

    pub fn range_apply(&mut self, a: usize, b: usize, v: M) {
        self.range_apply_sub(0, a, b, 0, 0, v)
    }

    pub fn point_set(&mut self, i: usize, v: X) {
        self.range_apply(i, i + 1, self.em.clone());
        self.dat[i + self.n - 1] = v;
    }
}
