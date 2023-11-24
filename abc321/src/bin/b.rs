// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n - 1]
    }
    let mut a = a;
    a.sort();
    // if n == 3 {
    //     if a[0] >= x {
    //         println!("0")
    //     } else if a[1] >= x {
    //         println!("{}", x);
    //     } else {
    //         println!("-1");
    //     }
    //     return;
    // }
    let sum: usize = a.clone().iter().sum();
    if sum - a[0] - a[n - 2] >= x {
        println!("0");
        return;
    }
    if sum - a[n - 2] >= x {
        println!("0");
        return;
    }
    if sum - a[0] >= x {
        println!("{}", x - (sum - a[0] - a[n - 2]));
        return;
    }
    println!("-1");
}
