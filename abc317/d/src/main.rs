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
        XYZ: [(u128, u128, usize); N],
    }
    let mut need: i32 = 0;
    let mut data = vec![];
    for (_, _, z) in XYZ.clone() {
        need += z as i32;
    }
    need = need / 2 + 1;
    for (x, y, z) in XYZ.clone() {
        if x > y {
            need -= z as i32;
        } else {
            data.push(((x + y) / 2 + 1 - x, z));
        }
    }
    if need <= 0 {
        println!("0");
        return;
    }
    let mut dp: Vec<Vec<u128>> = vec![vec![0; 100001]; N];
    for j in 0..=100000 {
        let (w, v) = data[0];
        dp[0][j] = if j <= v { w } else { std::u128::MAX }
    }
    for i in 1..data.len() {
        let (w, v) = data[i];
        for j in 0..=100000 {
            if j <= v {
                dp[i][j] = if dp[i - 1][j] > w { w } else { dp[i - 1][j] }
            } else {
                dp[i][j] = if dp[i - 1][j - v] <= std::u128::MAX - w
                    && dp[i - 1][j - v] + w < dp[i - 1][j]
                {
                    dp[i - 1][j - v] + w
                } else {
                    dp[i - 1][j]
                }
            }
        }
    }
    println!("{}", dp[data.len() - 1][need as usize]);
}
