use proconio::{fastout, input};

fn all1(S: Vec<char>) -> i32 {
    let mut found: Vec<bool> = vec![false; 10];
    let mut ans = 0;
    for c in S {
        if !found[c as usize - '0' as usize] {
            ans += 1;
            found[c as usize - '0' as usize] = true;
        }
    }
    ans
}

fn all2(S: Vec<char>) -> i32 {
    let mut found: Vec<bool> = vec![false; 10];
    let mut ans = 0;
    for (i, c) in S.iter().enumerate() {
        if !found[*c as usize - '0' as usize] {
            ans += all1(S[(i + 1)..].to_vec());
            found[*c as usize - '0' as usize] = true;
        }
    }
    ans
}

fn all3(S: Vec<char>) -> i32 {
    let mut found: Vec<bool> = vec![false; 10];
    let mut ans = 0;
    for (i, c) in S.iter().enumerate() {
        if !found[*c as usize - '0' as usize] {
            ans += all2(S[(i + 1)..].to_vec());
            found[*c as usize - '0' as usize] = true;
        }
    }
    ans
}

#[fastout]
fn main() {
    input! {
        N: usize,
        S: String
    }
    let S: Vec<char> = S.chars().collect();
    println!("{}", all3(S));
}
