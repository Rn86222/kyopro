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

fn check_tak(S: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if S[y + i][x + j] == '.' || S[y + 6 + i][x + 6 + j] == '.' {
                return false;
            }
        }
    }
    for i in 0..4 {
        if S[y + i][x + 3] == '#' || S[y + 5 + i][x + 5] == '#' {
            return false;
        }
    }
    for i in 0..3 {
        if S[y + 3][x + i] == '#' || S[y + 5][x + 6 + i] == '#' {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        N: usize,
        M: usize,
        S: [chars; N],
    }
    for i in 0..=(N - 9) {
        for j in 0..=(M - 9) {
            if check_tak(&S, i, j) {
                println!("{} {}", i + 1, j + 1);
            }
        }
    }
}
