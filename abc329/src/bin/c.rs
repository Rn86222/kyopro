// use itertools::Itertools;
use proconio::{fastout, input};

fn get_code(c: char) -> usize {
    c as u8 as usize - 'a' as u8 as usize
}

#[fastout]
fn main() {
    input! {
        n: usize,
        s: String
    }

    let s = s.chars().collect::<Vec<char>>();
    let mut max_len = vec![0; 26];
    let mut pos = 0;
    while pos < n {
        let mut len = 0;
        let mut cnt = 0;
        let current = get_code(s[pos]);
        while pos + len < n && s[pos] == s[pos + len] {
            len += 1;
            cnt += 1;
        }
        max_len[current] = std::cmp::max(max_len[current], cnt);
        pos += len;
    }
    let mut ans = 0;
    for i in 0..26 {
        ans += max_len[i];
    }
    println!("{}", ans);
}
