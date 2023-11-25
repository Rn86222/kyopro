// use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        ss: [Chars; n],
    }
    let mut row_cnts = vec![0_usize; n];
    let mut col_cnts = vec![0_usize; n];
    for i in 0..n {
        for j in 0..n {
            if ss[i][j] == 'o' {
                row_cnts[i] += 1;
                col_cnts[j] += 1;
            }
        }
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            if ss[i][j] == 'o' {
                ans += (row_cnts[i] - 1) * (col_cnts[j] - 1);
            }
        }
    }
    println!("{}", ans);
}
