// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ss: [String; n]
    }

    let mut counts = vec![];
    for (i, s) in ss.clone().iter().enumerate() {
        let mut count: u32 = 0;
        for c in s.chars() {
            if c == 'o' {
                count += 1;
            }
        }
        counts.push((i + 1, count));
    }

    counts.sort_by(|x, y| y.1.cmp(&x.1));
    for (i, _) in counts {
        println!("{}", i);
    }
}
