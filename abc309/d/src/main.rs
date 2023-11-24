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

use std::collections::VecDeque;

fn main() {
    input! {
        N1: usize,
        N2: usize,
        M: usize,
        ab: [(usize, usize); M],
    }
    let mut neigh: Vec<Vec<usize>> = vec![vec![]; N1 + N2];
    for (a, b) in ab.clone() {
        neigh[a - 1].push(b);
        neigh[b - 1].push(a);
    }
    let mut q1: VecDeque<usize> = VecDeque::new();
    q1.push_back(1);
    let mut dist1: Vec<i32> = vec![-1; N1 + N2];
    dist1[0] = 0;
    while q1.len() > 0 {
        let v = q1.pop_front().unwrap();
        let v_neigh = neigh[v - 1].clone();
        for n in v_neigh {
            if dist1[n - 1] == -1 {
                dist1[n - 1] = dist1[v - 1] + 1;
                q1.push_back(n);
            }
        }
    }
    let dist1_max = *dist1.iter().max().unwrap();

    let mut q2: VecDeque<usize> = VecDeque::new();
    q2.push_back(N1 + N2);
    let mut dist2: Vec<i32> = vec![-1; N1 + N2];
    dist2[N1 + N2 - 1] = 0;
    while q2.len() > 0 {
        let v = q2.pop_front().unwrap();
        let v_neigh = neigh[v - 1].clone();
        for n in v_neigh {
            if dist2[n - 1] == -1 {
                dist2[n - 1] = dist2[v - 1] + 1;
                q2.push_back(n);
            }
        }
    }
    let dist2_max = *dist2.iter().max().unwrap();
    println!("{}", dist1_max + dist2_max + 1);
}
