use proconio::{fastout, input};
// use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m]
    }
    let mut a_cnt = 0;
    let mut current_a = a[0];
    for i in 1..=n {
        while current_a < i {
            a_cnt += 1;
            current_a = a[a_cnt];
        }
        println!("{}", current_a - i);
    }
}
