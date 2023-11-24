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

struct Ternary(u128);

impl Ternary {
    fn count_ones(&self) -> u128 {
        let mut n = self.0;
        let mut count = 0;
        while n > 0 {
            count += n % 3;
            n /= 3;
        }
        count
    }
}

fn main() {
    input! {
        N: i32,
        AB: [(u128, u128); N]
    }
    let M: Vec<u128> = AB
        .iter()
        .map(|(a, b)| if *a > *b { *a } else { *b })
        .collect();
    let c: Vec<bool> = AB.iter().map(|(a, b)| *a > *b).collect();
    let d: Vec<i128> = AB
        .iter()
        .map(|(a, b)| {
            if *a > *b {
                *a as i128 - *b as i128
            } else {
                *b as i128 - *a as i128
            }
        })
        .collect();
    let mut count = 0;
    let mut ans = 0;
    for i in 0..N {
        if c[i as usize] {
            count += 1;
        }
        ans += M[i as usize];
    }
    if count % 2 == 0 {
        println!("{}", ans);
    } else {
        let mut min_d = d[0];
        for i in 1..N {
            if min_d > d[i as usize] {
                min_d = d[i as usize];
            }
        }
        println!("{}", ans as i128 - min_d);
    }
}
