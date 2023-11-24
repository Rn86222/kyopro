// use itertools::Itertools;
use proconio::{fastout, input};

fn is_ok(index: i64, v: i128, b: &Vec<i128>, p: i128) -> bool {
    return v + b[index as usize] > p;
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        p: i128,
        a: [i128; n],
        b: [i128; m]
    }
    let mut a = a;
    let mut b = b;
    a.sort();
    b.sort();
    let mut ans = 0;
    let mut rui = vec![];
    for i in 0..m {
        if i == 0 {
            rui.push(b[i]);
        } else {
            rui.push(b[i] + rui[i - 1]);
        }
    }

    for v in a {
        let mut ok: i64 = -1;
        let mut ng: i64 = m as i64;

        while ng - ok > 1 {
            let mid = ok + (ng - ok) / 2;
            if is_ok(mid, v, &b, p) {
                ng = mid;
            } else {
                ok = mid;
            }
        }
        if ok >= 0 {
            ans += rui[ok as usize] + v * (ok as i128 + 1);
        }
        if ng <= (m - 1) as i64 {
            ans += (p as i64 * (m as i64 - ng)) as i128;
        }
    }
    println!("{}", ans);
}
