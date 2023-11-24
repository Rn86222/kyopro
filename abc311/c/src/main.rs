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
use std::collections::HashMap;

fn main() {
    input! {
        N: u64,
        A: [usize; N],
    }
    let mut v: usize = 0;
    let mut hash: HashMap<usize, usize> = HashMap::new();
    let mut M = 0;
    let mut idx = 0;
    hash.insert(v, 0);
    for i in 1..=N {
        let goal = A[v] - 1;
        if let Some(found) = hash.get(&goal) {
            M = i as usize - *found;
            idx = goal;
            break;
        } else {
            hash.insert(goal, i as usize);
            v = goal;
        }
    }
    println!("{}", M);
    for _ in 0..M {
        print!("{} ", idx + 1);
        idx = A[idx] - 1;
    }
    println!("");
}
