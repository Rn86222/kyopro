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
        A: [chars; N],
    }

    let mut B = A.clone();
    for i in 1..N {
        B[0][i as usize] = A[0][(i - 1) as usize];
    }
    for i in 1..N {
        B[i as usize][(N - 1) as usize] = A[(i - 1) as usize][(N - 1) as usize];
    }
    for i in 0..(N - 1) {
        B[(N - 1) as usize][i as usize] = A[(N - 1) as usize][(i + 1) as usize];
    }
    for i in 0..(N - 1) {
        B[i as usize][0] = A[(i + 1) as usize][0];
    }
    for i in 0..N {
        for j in 0..N {
            print!("{}", B[i as usize][j as usize]);
        }
        println!("");
    }
}
