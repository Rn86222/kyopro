// use itertools::Itertools;
use proconio::{fastout, input};

fn calc(qs: &Vec<usize>, k: usize) -> f64 {
    let mut sum1 = 0.;
    let mut sum2 = 0.;
    for i in 1..=k {
        let a: f64 = 0.9;
        let power = a.powf((k - i) as f64);
        sum1 += power * qs[i - 1] as f64;
        sum2 += power;
    }
    return sum1 / sum2 - 1200. / (k as f64).sqrt();
}

#[fastout]
fn main() {
    input! {
        n: usize,
        ps: [usize; n],
    }
    let mut ps_with_index = ps.iter().enumerate().collect::<Vec<_>>();
    ps_with_index.sort_by(|a, b| b.1.cmp(a.1));
    // println!("{:?}", ps_with_index);
    let mut max_rate = std::f64::MIN;
    for k in 1..=n {
        let mut qs = vec![];
        for j in 0..k {
            qs.push(ps_with_index[j]);
        }
        qs.sort_by(|a, b| a.0.cmp(&b.0));
        let qs = qs.iter().map(|x| *x.1).collect::<Vec<_>>();
        let rate = calc(&qs, k);
        // println!("{} {}", k, rate);
        if max_rate < rate {
            max_rate = rate;
        }
    }
    println!("{}", max_rate);
}
