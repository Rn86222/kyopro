use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        A: u128,
        B: u128,
        C: u128,
        X: u128,
        Y: u128
    }
    let Z = if X < Y { X } else { Y };
    if A + B < 2 * C {
        println!("{}", A * X + B * Y);
        return;
    }
    let mut ans = 0;
    ans += 2 * C * Z;
    if X < Y {
        if B < 2 * C {
            ans += B * (Y - Z);
        } else {
            ans += 2 * C * (Y - Z);
        }
    } else {
        if A < 2 * C {
            ans += A * (X - Z);
        } else {
            ans += 2 * C * (X - Z);
        }
    }
    println!("{}", ans);
}
