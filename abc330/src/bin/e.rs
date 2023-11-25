// use itertools::Itertools;
use proconio::{fastout, input};
use std::collections::{BTreeSet, HashMap};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        qs: [(usize, i32); q]
    }
    let mut a = a;
    let mut s = NekoSet::new();
    for i in 0..n {
        s.insert(a[i] as i32);
    }
    for (i, x) in qs {
        let i = i - 1;
        let x = x as i32;
        let old = a[i] as i32;
        a[i] = x as usize;
        s.delete(old);
        s.insert(x);
        println!("{}", s.mex(0));
    }
}

struct NekoSet {
    s: BTreeSet<(i32, i32)>,
    counts: HashMap<i32, i32>,
}
impl NekoSet {
    fn new() -> Self {
        Self {
            s: vec![
                (std::i32::MIN, std::i32::MIN),
                (std::i32::MAX, std::i32::MAX),
            ]
            .into_iter()
            .collect(),
            counts: HashMap::new(),
        }
    }
    fn insert(&mut self, x: i32) -> bool {
        if self.counts.contains_key(&x) {
            let c = self.counts.get(&x).unwrap();
            self.counts.insert(x, *c + 1);
        } else {
            self.counts.insert(x, 1);
        }
        let &(nl, nu) = self.s.range((x + 1, x + 1)..).next().unwrap();
        let &(l, u) = self.s.range(..(x + 1, x + 1)).next_back().unwrap();
        if l <= x && x <= u {
            return false;
        }
        if u == x - 1 {
            if nl == x + 1 {
                self.s.remove(&(nl, nu));
                self.s.remove(&(l, u));
                self.s.insert((l, nu));
            } else {
                self.s.remove(&(l, u));
                self.s.insert((l, x));
            }
        } else {
            if nl == x + 1 {
                self.s.remove(&(nl, nu));
                self.s.insert((x, nu));
            } else {
                self.s.insert((x, x));
            }
        }
        true
    }
    fn mex(&self, x: i32) -> i32 {
        let &(l, u) = self.s.range(..(x + 1, x + 1)).next_back().unwrap();
        if l <= x && x <= u {
            u + 1
        } else {
            x
        }
    }

    fn delete(&mut self, x: i32) {
        if self.counts.contains_key(&x) {
            let c = self.counts.get(&x).unwrap();
            if *c > 1 {
                self.counts.insert(x, *c - 1);
            } else {
                self.counts.remove(&x);
            }
        }
        let contain = self.counts.contains_key(&x);

        if contain {
            return;
        }
        let &(l, u) = self.s.range(..(x + 1, x + 1)).next_back().unwrap();
        if l <= x && x <= u {
            if l == x {
                if u == x {
                    self.s.remove(&(l, u));
                } else {
                    self.s.remove(&(l, u));
                    self.s.insert((x + 1, u));
                }
            } else {
                if u == x {
                    self.s.remove(&(l, u));
                    self.s.insert((l, x - 1));
                } else {
                    self.s.remove(&(l, u));
                    self.s.insert((l, x - 1));
                    self.s.insert((x + 1, u));
                }
            }
        }
    }
}
