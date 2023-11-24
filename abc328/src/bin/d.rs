// use std::io::{stdout, Write};

use itertools::Itertools;
use proconio::input;

// #[fastout]
fn main() {
    input! {
        s: String,
    }
    let s = s.chars().collect_vec();
    let mut ans = vec![];

    for c in s {
        ans.push(c);
        let len = ans.len();
        if len >= 3 && ans[len - 3] == 'A' && ans[len - 2] == 'B' && ans[len - 1] == 'C' {
            ans.pop();
            ans.pop();
            ans.pop();
        }
    }

    for c in ans {
        print!("{}", c);
    }
    println!("");
    // stdout().flush().unwrap();
}
