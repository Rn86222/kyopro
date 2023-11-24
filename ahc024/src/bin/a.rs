use proconio::{fastout, input};
// use std::collections::HashSet;

fn check(i: usize, j: usize, cmap: &Vec<Vec<usize>>, before_center_color: usize) -> bool {
    let ok_pos = vec![
        vec![(0, 1)],
        vec![(1, 0)],
        vec![(2, 1)],
        vec![(1, 2)],
        vec![(0, 0), (0, 1)],
        vec![(0, 1), (0, 2)],
        vec![(0, 0), (1, 0)],
        vec![(1, 0), (2, 0)],
        vec![(2, 0), (2, 1)],
        vec![(2, 1), (2, 2)],
        vec![(0, 2), (1, 2)],
        vec![(1, 2), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2)],
        vec![(0, 0), (1, 0), (2, 0)],
        vec![(2, 0), (2, 1), (2, 2)],
        vec![(0, 2), (1, 2), (2, 2)],
        vec![(0, 0), (0, 1), (1, 0)],
        vec![(0, 1), (0, 2), (1, 2)],
        vec![(1, 0), (2, 0), (2, 1)],
        vec![(1, 2), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (1, 0)],
        vec![(0, 0), (0, 1), (1, 0), (2, 0)],
        vec![(0, 1), (0, 2), (1, 2), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1)],
        vec![(1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(1, 2), (2, 0), (2, 1), (2, 2)],
        vec![(0, 2), (1, 2), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (1, 0), (2, 0)],
        vec![(0, 2), (1, 2), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 2)],
        vec![(0, 0), (0, 1), (1, 0), (2, 0), (2, 1)],
        vec![(0, 1), (0, 2), (1, 2), (2, 1), (2, 2)],
        vec![(1, 0), (1, 2), (0, 2), (2, 1), (2, 2)],
    ];
    let mut white_pos = vec![];
    for dy in 0..=2 {
        for dx in 0..=2 {
            if dy * dx != 1 && cmap[i + dy - 1][j + dx - 1] == 0 {
                white_pos.push((dy, dx));
            }
            if cmap[i + dy - 1][j + dx - 1] != 0
                && cmap[i + dy - 1][j + dx - 1] != before_center_color
            {
                return false;
            }
        }
    }
    for ok in ok_pos {
        if white_pos == ok {
            return true;
        }
    }
    return false;
}

#[fastout]
fn main() {
    input! {
        n: usize,
        _: usize,
        cmap: [[usize; n]; n]
    }
    let mut cmap = cmap;
    for i in 0..n {
        let mut line = cmap[i].clone();
        line.insert(0, 0);
        line.push(0);
        cmap[i] = line;
    }
    cmap.insert(0, vec![0; n + 2]);
    cmap.push(vec![0; n + 2]);
    for i in 1..=(n / 2 - 1) {
        for j in i..=(n - i + 1) {
            let current_color = cmap[i][j];
            cmap[i][j] = 0;
            if !check(i, j, &cmap, current_color) {
                cmap[i][j] = current_color;
            }
            let current_color = cmap[j][i];
            cmap[j][i] = 0;
            if !check(j, i, &cmap, current_color) {
                cmap[j][i] = current_color;
            }
            let current_color = cmap[j][n - i + 1];
            cmap[j][n - i + 1] = 0;
            if !check(j, n - i + 1, &cmap, current_color) {
                cmap[j][n - i + 1] = current_color;
            }
            let current_color = cmap[n - i + 1][j];
            cmap[n - i + 1][j] = 0;
            if !check(n - i + 1, j, &cmap, current_color) {
                cmap[n - i + 1][j] = current_color;
            }
        }
    }
    for i in 1..=n {
        for j in 1..=n {
            print!("{} ", cmap[i][j]);
        }
        println!("");
    }
}
