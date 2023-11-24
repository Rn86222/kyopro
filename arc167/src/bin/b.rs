use proconio::{fastout, input};

// use itertools::Itertools;

const M: u128 = 998244353;

#[fastout]
fn main() {
    input! {
        a: u128,
        b: u128,
    }
    let mut a = a;
    let mut prime_factors = vec![];
    for p in 2.. {
        if p * p > a {
            break;
        }
        if a % p == 0 {
            let mut cnt = 0;
            while a % p == 0 {
                a /= p;
                cnt += 1;
            }
            prime_factors.push((p, cnt));
        }
    }
    if a > 1 {
        prime_factors.push((a, 1));
    }

    let mut odd = true;
    for (_, cnt) in prime_factors.iter() {
        if cnt % 2 == 1 {
            odd = false;
        }
    }
    if b % 2 == 0 {
        odd = false;
    }

    let mut ans = 1;
    for (_, cnt) in prime_factors.iter() {
        ans *= (cnt * b + 1) % M;
        ans %= M;
    }

    ans *= b;
    if odd {
        ans -= 1;
    }
    ans *= 499122177;
    ans %= M;

    println!("{}", ans % M);
}
