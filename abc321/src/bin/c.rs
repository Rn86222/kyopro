// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: usize
    }
    let mut digit: usize = 1;
    let mut likes = vec![];
    likes.push(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    while digit <= 9 {
        let mut current_likes = vec![];
        let before_likes = likes[digit - 1].clone();
        for l in before_likes {
            for i in (l / (10_usize.pow(digit as u32 - 1)) + 1)..=9 {
                current_likes.push(l + i * 10_usize.pow(digit as u32));
            }
        }
        likes.push(current_likes);
        digit += 1;
    }
    let mut cnt: i32 = -1;
    for i in 0..10 {
        likes[i].sort();
    }
    // println!("{:?}", likes);
    // println!("{}", likes.len());
    for like in likes {
        for l in like {
            cnt += 1;
            if cnt == k as i32 {
                println!("{}", l);
            }
        }
    }
}
