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

fn stou128(s: &Vec<char>) -> u128 {
    let mut ans = 0;
    for (i, c) in s.iter().enumerate() {
        if *c == 'o' {
            ans |= 1 << i;
        }
    }
    ans
}

fn main() {
    input! {
        N: usize,
        D: usize,
        S: [chars; N],
    }
    let U: Vec<u128> = S.iter().map(stou128).collect();
    let mut intersection: u128 = !0;
    for u in U {
        intersection &= u;
    }
    let mut ans = 0;
    let mut tmp = 0;
    for i in 0..D {
        if (intersection >> i) & 1 == 0 {
            if ans < tmp {
                ans = tmp;
            }
            tmp = 0;
        } else {
            tmp += 1;
        }
    }
    if ans < tmp {
        ans = tmp;
    }
    println!("{}", ans);
}
