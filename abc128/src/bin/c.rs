use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize
    }
    let mut s = vec![];
    for _ in 0..m {
        input! {
            k: usize,
            si: [usize; k]
        }
        s.push(si);
    }
    input! {
        p: [usize; m]
    }

    let mut ans = 0;
    let power = 2_u32.pow(n as u32);
    for i in 0..power {
        let mut ok = true;
        for j in 0..m {
            let mut tmp = 0;
            for sw in s[j].clone() {
                if i >> (sw - 1) & 1 == 1 {
                    tmp += 1;
                }
            }
            if tmp % 2 != p[j] {
                ok = false;
            }
        }
        if ok {
            ans += 1;
        }
    }
    println!("{}", ans);
}
