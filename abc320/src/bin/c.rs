use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        m: usize,
        s1: String,
        s2: String,
        s3: String,
    }
    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.chars().collect();
    let s3: Vec<char> = s3.chars().collect();
    let s = vec![s1, s2, s3];
    let mut ans: i32 = -1;

    for perm in (0..3).permutations(3) {
        for i in 0..10 {
            let mut found = false;
            let mut first = 0;
            for j in 0..m {
                if s[perm[0]][j] as u8 == '0' as u8 + i {
                    found = true;
                    first = j;
                    break;
                }
            }
            if !found {
                continue;
            }
            found = false;
            let mut second = 0;
            for j in 0..(2 * m) {
                if s[perm[1]][j % m] as u8 == '0' as u8 + i && j != first {
                    found = true;
                    second = j;
                    break;
                }
            }
            if !found {
                continue;
            }
            found = false;
            let mut third = 0;
            for j in 0..(3 * m) {
                if s[perm[2]][j % m] as u8 == '0' as u8 + i && j != first && j != second {
                    found = true;
                    third = j;
                    break;
                }
            }
            if !found {
                continue;
            }
            let mut tmp = first;
            if tmp < second {
                tmp = second;
            }
            if tmp < third {
                tmp = third;
            }
            if (tmp as i32) < ans || ans == -1 {
                ans = tmp as i32;
            }
        }
    }

    println!("{}", ans);
}
