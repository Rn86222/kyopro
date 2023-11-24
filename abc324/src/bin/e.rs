use proconio::{fastout, input};
// use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        n: usize,
        t: String,
        ss: [String; n],
    }

    let t = t.chars().collect::<Vec<char>>();
    let mut tmp_ss = vec![];
    for s in ss {
        tmp_ss.push(s.chars().collect::<Vec<char>>());
    }
    let mut begin_cnts = vec![];
    let mut end_cnts = vec![];

    let ss = tmp_ss;
    for s in ss {
        let mut i = 0;
        let mut j = 0;
        let mut begin_cnt = 0;
        while i < t.len() && j < s.len() {
            if t[i] == s[j] {
                i += 1;
                j += 1;
                begin_cnt += 1;
            } else {
                j += 1;
            }
        }
        begin_cnts.push(begin_cnt);
        let mut i = t.len() - 1;
        let mut j = s.len() - 1;
        let mut end_cnt = 0;
        loop {
            if t[i] == s[j] {
                end_cnt += 1;
                if i == 0 || j == 0 {
                    break;
                }
                i -= 1;
                j -= 1;
            } else {
                if j == 0 {
                    break;
                }
                j -= 1;
            }
        }
        end_cnts.push(end_cnt);
    }
    begin_cnts.sort();
    end_cnts.sort();
    end_cnts.reverse();

    let mut ans = 0;
    let mut i = 0;
    let mut j = 0;
    while i < begin_cnts.len() && j < end_cnts.len() {
        if begin_cnts[i] + end_cnts[j] >= t.len() {
            ans += begin_cnts.len() - i;
            j += 1;
        } else {
            i += 1;
        }
    }
    println!("{}", ans);
}
