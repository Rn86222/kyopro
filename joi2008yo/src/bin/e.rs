use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        r: usize,
        c: usize,
        a: [[usize; c]; r]
    }
    let bit = 2_u32.pow(r as u32);
    let mut ans = 0;
    for i in 0..bit {
        let mut tmp = 0;
        for col in 0..c {
            let mut col_zero_cnt = 0;
            for row in 0..r {
                if (a[row][col] == 1 && (i >> row & 1 == 1))
                    || (a[row][col] == 0 && (i >> row & 1 == 0))
                {
                    col_zero_cnt += 1;
                }
            }
            if col_zero_cnt > r / 2 {
                tmp += col_zero_cnt;
            } else {
                tmp += r - col_zero_cnt;
            }
        }
        if tmp > ans {
            ans = tmp;
        }
    }
    println!("{}", ans);
}
