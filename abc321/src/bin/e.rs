// use itertools::Itertools;
use proconio::{fastout, input};

fn search(x: u128, k: u128, n: u128) -> u128 {
    if k > 60 || (x << k > n) {
        return 0;
    }
    if ((x + 1) << k) - 1 > n {
        return n - (x << k) + 1;
    }
    return 1 << k as u128;
}

#[fastout]
fn main() {
    input! {
        t: usize,
        nxk: [(u128, u128, u128); t]
    }
    for (n, mut x, mut k) in nxk {
        let mut ans = 0;
        ans += search(x, k, n);
        while x > 1 && k >= 2 {
            if x % 2 == 0 {
                ans += search(x + 1, k - 2, n);
            } else {
                ans += search(x - 1, k - 2, n);
            }
            x /= 2;
            k -= 1;
        }
        if k == 1 && x > 1 {
            ans += 1;
        }
        println!("{}", ans);
    }
}
