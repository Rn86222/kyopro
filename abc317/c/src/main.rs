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

fn dfs(i: usize, edges: &Vec<Vec<(usize, u128)>>, history: usize) -> u128 {
    let mut max = 0;
    for (d, c) in edges[i].clone() {
        if history & (1 << d) != 0 {
            continue;
        }
        let tmp = dfs(d, edges, history | (1 << d));
        if max < tmp + c {
            max = tmp + c;
        }
    }
    max
}

fn main() {
    input! {
        N: usize,
        M: usize,
        ABC: [(usize, usize, u128); M],
    }
    let mut edges: Vec<Vec<(usize, u128)>> = vec![vec![]; N + 1];
    for (a, b, c) in ABC {
        edges[a].push((b, c));
        edges[b].push((a, c));
    }
    let mut max = 0;
    for i in 1..=N {
        let tmp = dfs(i, &edges, 1 << i);
        if tmp > max {
            max = tmp;
        }
    }
    println!("{}", max);
}
