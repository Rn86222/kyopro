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

fn update_grid(grid: &mut Vec<Vec<char>>, start_x: i32, start_y: i32, dx: i32, dy: i32) {
    if grid[(start_y + dy) as usize][(start_x + dx) as usize] == '#' {
        return;
    }
    let mut current_x = start_x;
    let mut current_y = start_y;
    let mut before_val = '.';

    loop {
        if grid[(current_y + dy) as usize][(current_x + dx) as usize] == '.'
            || grid[(current_y + dy) as usize][(current_x + dx) as usize] == 's'
        {
            before_val = grid[(current_y + dy) as usize][(current_x + dx) as usize];
            grid[(current_y + dy) as usize][(current_x + dx) as usize] = 's';
            current_y += dy;
            current_x += dx;
        } else {
            if before_val != 's' {
                update_grid(grid, current_x, current_y, 1, 0);
                update_grid(grid, current_x, current_y, -1, 0);
                update_grid(grid, current_x, current_y, 0, 1);
                update_grid(grid, current_x, current_y, 0, -1);
            }
            return;
        }
    }
}

fn main() {
    input! {
        N: usize,
        M: usize,
        grid: [chars; N],
    }
    let mut grid = grid.clone();
    grid[1][1] = 's';
    update_grid(&mut grid, 1, 1, 1, 0);
    update_grid(&mut grid, 1, 1, 0, 1);
    let mut ans = 0;
    for i in 0..N {
        for j in 0..M {
            if grid[i][j] == 's' {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
