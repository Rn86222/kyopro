// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        an: [usize; n],
    }
    let mut an = an;
    an.sort();
    an.reverse();
    let mut dishes = vec![0; m];
    let mut ans = 0;
    // let diff = n - m;
    // let mut toast = 0;
    let mut dish = 0;
    for i in 0..m {
        dishes[m - i - 1] = an[i];
    }
    for i in m..n {
        dishes[dish] += an[i];
        dish += 1;
    }

    // loop {
    //     if toast < (2 * diff) {
    //         dishes[dish] = an[toast] + an[toast + 1];
    //         dish += 1;
    //         toast += 2;
    //     } else {
    //         dishes[dish] = an[toast];
    //         dish += 1;
    //         toast += 1;
    //     }
    //     if toast >= n {
    //         break;
    //     }
    // }
    // println!("{:?}", dishes);
    for d in dishes {
        ans += d * d;
    }
    println!("{}", ans);
}
