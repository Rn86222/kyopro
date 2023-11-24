// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
        ns: [u64; t]
    }

    for n in ns {
        let tmp = n;
        let mut n = n;
        let mut primes = vec![];
        for i in 2..=((tmp as f64).sqrt() as u64 + 1) {
            if n % i == 0 {
                let mut count: u32 = 0;
                while n % i == 0 {
                    n /= i;
                    count += 1;
                }
                primes.push((i, count));
            }
        }
        if n != 1 {
            primes.push((n, 1))
        }
        let mut sum = 0;
        for (p, k) in primes.clone() {
            sum += p.pow(k);
        }
        if primes.len() > 1 && sum <= tmp {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
