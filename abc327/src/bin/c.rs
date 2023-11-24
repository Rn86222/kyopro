// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: [[usize; 9]; 9],
    }
    for i in 0..9 {
        let mut line: u32 = 0;
        for j in 0..9 {
            line |= 1 << a[i][j];
        }
        if line.count_ones() != 9 {
            println!("No");
            return;
        }
    }
    for i in 0..9 {
        let mut line: u32 = 0;
        for j in 0..9 {
            line |= 1 << a[j][i];
        }
        if line.count_ones() != 9 {
            println!("No");
            return;
        }
    }
    for i in 0..3 {
        for j in 0..3 {
            let mut square: u32 = 0;
            for k in 0..3 {
                for l in 0..3 {
                    square |= 1 << a[i * 3 + k][j * 3 + l];
                }
            }
            if square.count_ones() != 9 {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
