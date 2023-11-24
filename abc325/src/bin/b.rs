use proconio::{fastout, input};
// use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        n: usize,
        wxs: [(usize, usize); n],
    }

    let mut counts = vec![0; 24];
    for i in 0..n {
        let (w, x) = wxs[i];
        for j in 9..18 {
            counts[(x + j) % 24] += w;
        }
    }
    let mut ans = 0;
    for c in counts {
        if ans < c {
            ans = c;
        }
    }
    println!("{}", ans);
}
