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

fn check_row(i: usize, row: &Vec<Vec<usize>>, rem_clm: usize) -> bool {
    if rem_clm < 2 {
        return false;
    }
    for color in 0..26 {
        if row[i][color as usize] == rem_clm {
            return true;
        }
    }
    false
}

fn check_clm(j: usize, clm: &Vec<Vec<usize>>, rem_row: usize) -> bool {
    if rem_row < 2 {
        return false;
    }
    for color in 0..26 {
        if clm[j][color as usize] == rem_row {
            return true;
        }
    }
    false
}

fn main() {
    input! {
        H: usize,
        W: usize,
        c: [chars; H],
    }
    let mut c = c.clone();
    let mut row: Vec<Vec<usize>> = vec![vec![0; 26]; H];
    let mut clm: Vec<Vec<usize>> = vec![vec![0; 26]; W];
    for i in 0..H {
        for j in 0..W {
            row[i][(c[i][j] as u8 - 'a' as u8) as usize] += 1;
            clm[j][(c[i][j] as u8 - 'a' as u8) as usize] += 1;
        }
    }
    let mut is_marked_row: Vec<bool> = vec![false; H];
    let mut is_marked_clm: Vec<bool> = vec![false; W];
    let mut rem_row = H;
    let mut rem_clm = W;
    loop {
        let mut marked_row: Vec<usize> = vec![];
        let mut marked_clm: Vec<usize> = vec![];
        for i in 0..H {
            if !is_marked_row[i] && check_row(i, &row, rem_clm) {
                is_marked_row[i] = true;
                marked_row.push(i);
            }
        }
        for j in 0..W {
            if !is_marked_clm[j] && check_clm(j, &clm, rem_row) {
                is_marked_clm[j] = true;
                marked_clm.push(j);
            }
        }
        if marked_row.len() == 0 && marked_clm.len() == 0 {
            break;
        }
        for i in marked_row {
            rem_row -= 1;
            for j in 0..W {
                let color = c[i][j];
                c[i][j] = 'A';
                if color != 'A' {
                    clm[j][color as usize - 'a' as usize] -= 1;
                }
            }
        }
        for j in marked_clm {
            rem_clm -= 1;
            for i in 0..H {
                let color = c[i][j];
                c[i][j] = 'A';
                if color != 'A' {
                    row[i][color as usize - 'a' as usize] -= 1;
                }
            }
        }
    }
    let mut ans = 0;
    for i in 0..H {
        for j in 0..W {
            if c[i][j] != 'A' {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
