use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: String
    }
    let n = n - 1;
    let s = s.chars().collect_vec();
    // let mut before_is_big = false;
    let mut current_pos = 0;
    let mut ans = 0;
    while current_pos < n {
        let c = s[current_pos];
        if c == '<' {
            // before_is_big = true;
            let mut i = 0;
            while current_pos + i < n && s[current_pos + i] == '<' {
                i += 1;
            }
            current_pos += i;
        } else {
            let mut cnt = 0;
            while current_pos + cnt < n && s[current_pos + cnt] == '>' {
                cnt += 1;
            }
            // println!("{} {}", current_pos, cnt);
            // if !before_is_big {
            //     ans += (cnt - 1) * cnt / 2;
            // } else {
            ans += cnt * (cnt + 1) / 2;
            // }
            current_pos += cnt;
        }
    }
    println!("{}", ans);
}
