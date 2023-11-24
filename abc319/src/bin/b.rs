use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize
    }
    let mut ans = vec![];
    for i in 0..=N {
        let mut c = '-';
        for j in 1..=9 {
            if N % j == 0 && i % (N / j) == 0 {
                c = (j as u8 + '0' as u8) as char;
                break;
            }
        }
        ans.push(c);
    }
    for c in ans {
        print!("{}", c);
    }
    println!("");
}
