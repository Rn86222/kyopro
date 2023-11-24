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

const snuke: &str = "snuke";

fn index(c: char) -> i32 {
    if c == 's' {
        0
    } else if c == 'n' {
        1
    } else if c == 'u' {
        2
    } else if c == 'k' {
        3
    } else if c == 'e' {
        4
    } else {
        5
    }
}

fn update(
    H: usize,
    W: usize,
    i: i32,
    j: i32,
    di: i32,
    dj: i32,
    info_map: &mut Vec<Vec<(u32, char)>>,
) {
    if i < 0 || i >= H as i32 || j < 0 || j >= W as i32 {
        return;
    }
    let (info, c) = info_map[i as usize][j as usize];
    if info == 1 {
        return;
    }
    let (_, c_) = info_map[(i + di) as usize][(j + dj) as usize];
    if (index(c) + 1) % 5 == index(c_) {
        info_map[i as usize][j as usize] = (1, c);
        update(H, W, i - 1, j, 1, 0, info_map);
        update(H, W, i + 1, j, -1, 0, info_map);
        update(H, W, i, j - 1, 0, 1, info_map);
        update(H, W, i, j + 1, 0, -1, info_map);
    }
}

fn main() {
    input! {
        H: usize,
        W: usize,
        S: [chars; H]
    }
    let mut info_map: Vec<Vec<(u32, char)>> = S
        .iter()
        .map(|line| line.iter().map(|c| (0, *c)).collect())
        .collect();
    let (_, c) = info_map[H - 1][W - 1];
    info_map[H - 1][W - 1] = (0, c);
    update(H, W, H as i32 - 2, W as i32 - 1, 1, 0, &mut info_map);
    update(H, W, H as i32 - 1, W as i32 - 2, 0, 1, &mut info_map);
    let (info, _) = info_map[0][0];
    println!("{}", if info == 1 { "Yes" } else { "No" });
}
