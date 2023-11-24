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

fn score(m: usize, e: usize, x: usize) -> usize {
    for i in 0..4 {
        if m != i && e != i && x != i {
            return i;
        }
    }
    return 3;
}

fn main() {
    input! {
        N: usize,
        A: [usize; N],
        S: chars
    }
    let mut dp: Vec<Vec<Vec<u64>>> = vec![vec![vec![0; 3]; N]; 27];
    let mut ans: u64 = 0;
    for t in 0..27 {
        let m = t / 9;
        let e = t % 9 / 3;
        let x = t % 3;
        if S[0] == 'M' && A[0] == m {
            dp[t][0][0] = 1;
        } else {
            dp[t][0][0] = 0;
        }
        dp[t][0][1] = 0;
        dp[t][0][2] = 0;
        for i in 1..N {
            dp[t][i][0] = dp[t][i - 1][0] + if S[i] == 'M' && A[i] == m { 1 } else { 0 };
            dp[t][i][1] = dp[t][i - 1][1]
                + if S[i] == 'E' && A[i] == e {
                    dp[t][i - 1][0]
                } else {
                    0
                };
            dp[t][i][2] = dp[t][i - 1][2]
                + if S[i] == 'X' && A[i] == x {
                    dp[t][i - 1][1]
                } else {
                    0
                };
        }
        ans += dp[t][N - 1][2] * score(m, e, x) as u64;
    }
    println!("{}", ans);
}
