use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        c: [[usize; 3]; 3]
    }
    let mut cnt = 0;
    let table = [
        vec![0, 3, 6],
        vec![0, 4],
        vec![0, 5, 7],
        vec![1, 3],
        vec![1, 4, 6, 7],
        vec![1, 5],
        vec![2, 3, 7],
        vec![2, 4],
        vec![2, 5, 6],
    ];
    for perm in (0..9).permutations(9) {
        let perm: Vec<usize> = perm;
        let mut check = vec![vec![]; 8];
        'outer: for m in perm {
            let lines = &table[m];
            for l in lines {
                let l = *l as usize;
                if check[l].len() == 2
                    && check[l][0] == check[l][1]
                    && check[l][0] != c[m / 3][m % 3]
                {
                    cnt += 1;
                    break 'outer;
                }
                check[l].push(c[m / 3][m % 3]);
            }
        }
    }
    println!("{}", 1. - cnt as f64 / 362880.);
}
