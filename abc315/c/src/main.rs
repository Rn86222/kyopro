macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    input! {
        N: u64,
        FS: [(u64, u64); N],
    }
    let mut max2: Vec<(u64, u64)> = vec![(0, 0), (0, 0)];
    let mut dp: Vec<u64> = vec![0; N as usize];
    for (i, (f, s)) in FS.iter().enumerate() {
        if i == 0 {
            max2[0] = (*f, *s);
            dp[0] = 0;
            continue;
        }
        let (first_f, first_s) = max2[0];
        let (second_f, second_s) = max2[1];
        let tmp1: u64;
        let tmp2: u64;
        if *f != first_f {
            tmp1 = *s + first_s;
        } else {
            if *s >= first_s {
                tmp1 = *s + first_s / 2;
            } else {
                tmp1 = *s / 2 + first_s;
            }
        }
        if *f != second_f {
            tmp2 = *s + second_s;
        } else {
            if *s >= second_s {
                tmp2 = *s + second_s / 2;
            } else {
                tmp2 = *s / 2 + second_s;
            }
        }
        dp[i] = if tmp1 >= tmp2 { tmp1 } else { tmp2 };
        if *f == first_f && *s > first_s {
            max2[0] = (*f, *s);
        } else if second_f == 0 || (*f == second_f && *s > second_s) {
            max2[1] = (*f, *s);
        } else if *s > first_s {
            if first_s < second_s {
                max2[0] = (*f, *s);
            } else {
                max2[1] = (*f, *s);
            }
        } else if *s > second_s {
            max2[1] = (*f, *s);
        }
    }
    let mut ans = 0;
    for s in dp.clone() {
        if ans < s {
            ans = s;
        }
    }
    println!("{}", ans);
}
