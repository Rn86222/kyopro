use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u128; n]
    }
    let bit = 2_u32.pow(n as u32);
    let mut ans = std::u128::MAX;
    for b in 0..bit {
        if b & 1 == 0 || b.count_ones() != k as u32 {
            continue;
        }
        let mut cost = 0;
        let mut max = a[0];
        for i in 1..n {
            if b >> i & 1 == 1 && max >= a[i] {
                cost += max + 1 - a[i];
                max = max + 1;
            } else if max < a[i] {
                max = a[i];
            }
        }
        if ans > cost {
            ans = cost;
        }
    }
    println!("{}", ans);
}
