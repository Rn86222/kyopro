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
        N: usize,
        K: u128,
        ab: [(u128, u128); N],
    }
    let mut sum: u128 = 0;
    let mut c: Vec<u128> = vec![0; N];
    let mut sort_ab = ab.clone();
    sort_ab.sort_by(|(a1, _), (a2, _)| a1.partial_cmp(a2).unwrap());
    for i in 0..N {
        let (_, bi) = sort_ab[i];
        sum += bi;
        c[i] = bi;
    }
    let mut dp: Vec<u128> = vec![0; N];
    let mut end = false;
    if sum <= K {
        println!("1");
        return;
    } else {
        dp[0] = c[0];
        for i in 1..N {
            dp[i] = dp[i - 1] + c[i];
            if dp[i] >= sum - K {
                let (ai, _) = sort_ab[i];
                println!("{}", ai + 1);
                end = true;
                break;
            }
        }
        if !end {
            let (last_a, _) = sort_ab[N - 1];
            println!("{}", last_a + 1);
        }
    }
}
