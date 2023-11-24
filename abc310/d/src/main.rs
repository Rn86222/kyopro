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

fn make(
    current_num: usize,
    bad: &Vec<u128>,
    teams: Vec<u128>,
    team_count: usize,
    T: usize,
    N: usize,
) -> i32 {
    if current_num == N {
        if team_count == T {
            return 1;
        } else {
            return 0;
        }
    }
    let mut ans = 0;
    for i in 0..team_count {
        if bad[current_num] & teams[i as usize] == 0 {
            let mut new_teams = teams.clone();
            new_teams[i as usize] |= (1 << current_num) as u128;
            ans += make(current_num + 1, bad, new_teams, team_count, T, N);
        }
    }
    if team_count < T {
        let mut new_teams = teams.clone();
        let new_team_count = team_count + 1;
        new_teams[new_team_count - 1] = 1 << current_num;
        ans += make(current_num + 1, bad, new_teams, new_team_count, T, N);
    }
    ans
}

fn main() {
    input! {
        N: usize,
        T: usize,
        M: usize,
        AB: [(usize, usize); M],
    }
    let mut bad: Vec<u128> = vec![0; N];
    let teams: Vec<u128> = vec![0; N];
    for (a, b) in AB {
        bad[b - 1] |= (1 << (a - 1)) as u128;
    }
    println!("{}", make(0, &bad, teams, 0, T, N));
}
