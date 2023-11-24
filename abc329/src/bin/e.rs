// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: String,
        t: String,
    }
    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();

    let mut before_pos = 0;
    let mut current_pos = 0;
    let mut before_is_end = true;
    while current_pos < n {
        let mut is_match = false;
        let mut start_pos_is_fixed = false;
        if !before_is_end {
            for start_pos_in_t in 0..(m - 1) {
                let back_step = (m - 1) - start_pos_in_t;
                if current_pos < back_step {
                    continue;
                }
                if current_pos - back_step <= before_pos {
                    continue;
                }
                let mut is_match = true;
                for i in 0..back_step {
                    if s[current_pos - back_step + i] != t[i] {
                        is_match = false;
                        break;
                    }
                }
                if is_match {
                    // println!("{}", back_step);
                    start_pos_is_fixed = true;
                    current_pos -= back_step;
                    break;
                }
            }
        }
        // println!("{} {} {}", current_pos, before_pos, before_is_end);
        before_pos = current_pos;
        if start_pos_is_fixed {
            let start_pos_in_t = 0;
            if s[current_pos] == t[start_pos_in_t] {
                is_match = true;
                let mut i = start_pos_in_t;
                while i < m && current_pos + i < n {
                    if s[current_pos + i] != t[i] {
                        break;
                    }
                    i += 1;
                }
                if current_pos + i == n {
                    if i != m {
                        println!("No");
                        return;
                    }
                }
                if i == m {
                    before_is_end = true;
                } else {
                    before_is_end = false;
                }
                current_pos += i;
            } else {
                panic!("start_pos_is_fixed {} {}", current_pos, start_pos_in_t);
            }
        } else {
            for start_pos_in_t in 0..m {
                if s[current_pos] == t[start_pos_in_t] {
                    is_match = true;
                    if !before_is_end && start_pos_in_t > 0 {
                        println!("No");
                        return;
                    }
                    let mut i = start_pos_in_t;
                    while i < m && current_pos + i < n {
                        if s[current_pos + i] != t[i] {
                            break;
                        }
                        i += 1;
                    }
                    if current_pos + i == n {
                        if i != m {
                            println!("No");
                            return;
                        }
                    }
                    if i == m {
                        before_is_end = true;
                    } else {
                        before_is_end = false;
                    }
                    current_pos += i;
                    break;
                }
            }
        }
        if !is_match {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
