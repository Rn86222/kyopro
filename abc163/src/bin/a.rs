use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        R: f64
    }
    println!("{}", 2. * R * std::f64::consts::PI);
}
