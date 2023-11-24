// use itertools::Itertools;
use proconio::{fastout, input};
use rand::Rng;
use std::time::Duration;
use std::time::Instant;

const DX: [i32; 4] = [1, 0, -1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

fn dfs(
    i: usize,
    j: usize,
    h: &Vec<Vec<char>>,
    v: &Vec<Vec<char>>,
    height: usize,
    width: usize,
    visited: &mut Vec<Vec<bool>>,
    used: &Vec<Vec<bool>>,
    gi: usize,
    gj: usize,
) {
    visited[i][j] = true;
    if i == gi && j == gj {
        return;
    }
    for dir in 0..4 {
        let ni = i as i32 + DY[dir];
        let nj = j as i32 + DX[dir];
        if (ni < 0 || ni >= height as i32 || nj < 0 || nj >= width as i32)
            || (DY[dir] == 1 && i < height - 1 && h[i][j] == '1')
            || (DY[dir] == -1 && i >= 1 && h[i - 1][j] == '1')
            || (DX[dir] == 1 && j < width - 1 && v[i][j] == '1')
            || (DX[dir] == -1 && j >= 1 && v[i][j - 1] == '1')
            || visited[ni as usize][nj as usize]
            || ((ni as usize != gi || nj as usize != gj) && used[ni as usize][nj as usize])
        {
            continue;
        }
        dfs(
            ni as usize,
            nj as usize,
            h,
            v,
            height,
            width,
            visited,
            used,
            gi,
            gj,
        );
    }
}

fn path_exists(
    h: &Vec<Vec<char>>,
    v: &Vec<Vec<char>>,
    si: usize,
    sj: usize,
    gi: usize,
    gj: usize,
    height: usize,
    width: usize,
    used: &Vec<Vec<bool>>,
) -> bool {
    let mut visited = vec![vec![false; width]; height];
    visited[si][sj] = true;
    dfs(si, sj, h, v, height, width, &mut visited, used, gi, gj);
    visited[gi][gj]
}

#[fastout]
fn main() {
    input! {
        t: usize,
        height: usize,
        width: usize,
        i0: usize,
        hs: [String; height - 1],
        vs: [String; width],
        k: usize,
        sd: [(usize, usize); k]
    }
    let start_time = Instant::now();

    let mut rnd = rand::thread_rng();
    let mut h = vec![];
    for line in hs {
        let line: Vec<char> = line.chars().collect();
        h.push(line);
    }
    let mut v = vec![];
    for line in vs {
        let line: Vec<char> = line.chars().collect();
        v.push(line);
    }

    let mut score: Vec<(usize, usize, usize, usize)> = sd
        .iter()
        .enumerate()
        .map(|(i, (s, d))| (d - s, *s, *d, i))
        .collect();
    let ht = vec![h.clone(); t];
    let vt = vec![v.clone(); t];
    let mut used = vec![vec![vec![false; width]; height]; t];
    score.sort();
    score.reverse();

    let mut ans = vec![];
    for (_, (_, s, d, index)) in score.iter().enumerate() {
        if start_time.elapsed() >= Duration::from_millis(1900) {
            break;
        }
        let mut i = height;
        let mut j = width;
        let mut ok = false;
        for _ in 0..3 {
            i = rnd.gen_range(0..height);
            j = rnd.gen_range(0..width);
            if i == i0 && j == 0 {
                continue;
            }
            if !path_exists(
                &ht[s - 1],
                &vt[s - 1],
                i0,
                0,
                i,
                j,
                height,
                width,
                &used[s - 1],
            ) || !path_exists(
                &ht[d - 1],
                &vt[d - 1],
                i0,
                0,
                i,
                j,
                height,
                width,
                &used[d - 1],
            ) {
                continue;
            }
            let mut found = false;
            for k in (s - 1)..*d {
                if used[k][i][j] {
                    found = true;
                    break;
                }
            }
            if found {
                continue;
            }
            ok = true;
            break;
        }
        if !ok {
            continue;
        }
        let mut found = false;
        for k in (s - 1)..*d {
            used[k][i][j] = true;
        }
        for (_, i, j, s, d) in ans.clone() {
            if !path_exists(
                &ht[s - 1],
                &vt[s - 1],
                i0,
                0,
                i,
                j,
                height,
                width,
                &used[s - 1],
            ) || !path_exists(
                &ht[d - 1],
                &vt[d - 1],
                i0,
                0,
                i,
                j,
                height,
                width,
                &used[d - 1],
            ) {
                found = true;
                break;
            }
        }
        if found {
            for k in (s - 1)..*d {
                used[k][i][j] = false;
            }
            continue;
        }
        ans.push((index + 1, i, j, s, d));
    }
    println!("{:?}", start_time.elapsed());
    println!("{}", ans.len());
    for (k, i, j, s, _) in ans {
        println!("{} {} {} {}", k, i, j, s);
    }
}
