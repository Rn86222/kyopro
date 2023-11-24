use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: String,
        ss: [String; n]
    }
    let mut anss = vec![];
    let t_chars = t.chars().collect_vec();

    for (k, s) in ss.iter().enumerate() {
        let s = (*s).clone();
        let s_chars = s.chars().collect_vec();

        if s == t {
            anss.push(k + 1);
        } else if s.len() == t.len() {
            let mut wrong_cnt = 0;
            for i in 0..s.len() {
                if s_chars[i] != t_chars[i] {
                    wrong_cnt += 1;
                }
            }
            if wrong_cnt == 1 {
                anss.push(k + 1);
            }
        } else if s.len() == t.len() + 1 {
            let mut wrong_cnt = 0;
            let mut i = 0;
            let mut j = 0;
            while i < s.len() && j < t.len() {
                if s_chars[i] != t_chars[j] {
                    wrong_cnt += 1;
                    if wrong_cnt > 1 {
                        break;
                    }
                    i += 1;
                } else {
                    i += 1;
                    j += 1;
                }
            }
            if wrong_cnt == 1 || (wrong_cnt == 0 && i == s.len() - 1) {
                anss.push(k + 1);
            }
        } else if s.len() + 1 == t.len() {
            let mut wrong_cnt = 0;
            let mut i = 0;
            let mut j = 0;
            while i < s.len() && j < t.len() {
                if s_chars[i] != t_chars[j] {
                    wrong_cnt += 1;
                    if wrong_cnt > 1 {
                        break;
                    }
                    j += 1;
                } else {
                    i += 1;
                    j += 1;
                }
            }
            if wrong_cnt == 1 || (wrong_cnt == 0 && j == t.len() - 1) {
                anss.push(k + 1);
            }
        }
    }
    println!("{}", anss.len());
    for ans in anss {
        print!("{} ", ans);
    }
    println!("");
}
