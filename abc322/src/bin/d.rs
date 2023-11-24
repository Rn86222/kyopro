use proconio::{fastout, input};

fn rotate(p: &mut Vec<Vec<char>>) {
    let mut rotated = vec![vec![' '; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            rotated[j][3 - i] = p[i][j];
        }
    }
    *p = rotated;
}

fn mapping(x: usize, y: usize, sharp_pos: &Vec<(usize, usize)>, map: &mut Vec<Vec<char>>) -> bool {
    let x = x as i32;
    let y = y as i32;

    let (start_x, start_y) = sharp_pos[0];
    let start_x = start_x as i32;
    let start_y = start_y as i32;
    let shift_x = x - start_x;
    let shift_y = y - start_y;
    for &(sharp_x, sharp_y) in sharp_pos {
        let current_x = sharp_x as i32 + shift_x;
        let current_y = sharp_y as i32 + shift_y;
        if current_x < 0 || current_x >= 4 || current_y < 0 || current_y >= 4 {
            return false;
        }
        if map[current_y as usize][current_x as usize] == '#' {
            return false;
        }
        map[current_y as usize][current_x as usize] = '#'
    }
    true
}

fn check(
    i0: usize,
    i1: usize,
    i2: usize,
    j0: usize,
    j1: usize,
    j2: usize,
    sharp_poss: &Vec<Vec<Vec<(usize, usize)>>>,
) -> bool {
    let is = vec![i0, i1, i2];
    let js = vec![j0, j1, j2];
    let mut map = vec![vec!['.'; 4]; 4];
    for k in 0..3 {
        let j = js[k];
        let x = j % 4;
        let y = j / 4;
        let i = is[k];
        if !mapping(x, y, &sharp_poss[k][i], &mut map) {
            return false;
        }
    }
    for i in 0..4 {
        for j in 0..4 {
            if map[i][j] == '.' {
                return false;
            }
        }
    }
    true
}

#[fastout]
fn main() {
    input! {
        p: [[String; 4]; 3]
    }
    let mut tmp_p = vec![];
    for i in 0..3 {
        let mut tmp: Vec<Vec<char>> = vec![];
        for j in 0..4 {
            tmp.push(p[i][j].chars().collect());
        }
        tmp_p.push(tmp);
    }
    let p = tmp_p;
    let mut sharp_poss = vec![];
    for i in 0..3 {
        let mut sharp_pos = vec![];
        for kaiten in 0..4 {
            let mut new_p = p[i].clone();
            for _ in 0..kaiten {
                rotate(&mut new_p);
            }
            let p = new_p;
            let mut tmp = vec![];
            for i in 0..4 {
                for j in 0..4 {
                    if p[i][j] == '#' {
                        tmp.push((j, i));
                    }
                }
            }
            sharp_pos.push(tmp);
        }
        sharp_poss.push(sharp_pos);
    }
    // for i0 in 0..4 {
    for i1 in 0..4 {
        for i2 in 0..4 {
            for j0 in 0..16 {
                for j1 in 0..16 {
                    for j2 in 0..16 {
                        if check(0, i1, i2, j0, j1, j2, &sharp_poss) {
                            println!("Yes");
                            return;
                        }
                    }
                }
            }
        }
    }
    // }
    println!("No");
}
