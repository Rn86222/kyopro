use proconio::{fastout, input};

fn time(x: f64, p: f64) -> f64 {
    if x < 0. {
        p
    } else {
        x + p / 2_f64.powf(x / 1.5)
    }
}

#[fastout]
fn main() {
    input! {
        p: f64
    }

    let mut left = 0.;
    let mut right = 100.;

    while left + 0.000000005 < right {
        let c1 = (2. * left + right) / 3.;
        let c2 = (left + 2. * right) / 3.;
        let fc1 = time(c1, p);
        let fc2 = time(c2, p);
        if fc1 > fc2 {
            left = c1;
        } else {
            right = c2;
        }
    }
    println!("{}", time((left + right) / 2., p));
}
