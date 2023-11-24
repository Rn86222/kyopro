use std::{collections::HashSet, vec};

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        r: String,
        c: String
    }
    let mut ans = vec![];
    let r = r.chars().collect::<Vec<char>>();
    let c = c.chars().collect::<Vec<char>>();
    let mut line = vec!['A', 'B', 'C'];
    for _ in 3..n {
        line.push('.');
    }
    let line_permutation = line.iter().permutations(n).collect::<Vec<_>>();
    let mut line_candidates = vec![vec![]; n];
    for i in 0..n {
        for l in line_permutation.clone() {
            if *l[0] == r[i]
                || (*l[0] == '.' && *l[1] == r[i])
                || (*l[0] == '.' && *l[1] == '.' && *l[2] == r[i])
            {
                line_candidates[i].push(l);
            }
        }
    }

    if n == 5 {
        'main: for l0 in 0..line_candidates[0].len() {
            for l1 in 0..line_candidates[1].len() {
                for l2 in 0..line_candidates[2].len() {
                    for l3 in 0..line_candidates[3].len() {
                        for l4 in 0..line_candidates[4].len() {
                            let map = vec![
                                &line_candidates[0][l0],
                                &line_candidates[1][l1],
                                &line_candidates[2][l2],
                                &line_candidates[3][l3],
                                &line_candidates[4][l4],
                            ];
                            let mut col_first_check = true;
                            for i in 0..n {
                                for j in 0..n {
                                    if *map[j][i] != c[i] && *map[j][i] != '.' {
                                        col_first_check = false;
                                        break;
                                    } else if *map[j][i] == c[i] {
                                        break;
                                    }
                                }
                                if !col_first_check {
                                    break;
                                }
                            }
                            if !col_first_check {
                                continue;
                            }
                            let mut col_second_check = true;
                            let mut ok_column_count = 0;
                            for i in 0..n {
                                let mut set = HashSet::new();
                                for j in 0..n {
                                    if *map[j][i] != '.' {
                                        if set.contains(&map[j][i]) {
                                            col_second_check = false;
                                            break;
                                        } else {
                                            set.insert(map[j][i]);
                                        }
                                    }
                                }
                                if !col_second_check || set.len() != 3 {
                                    break;
                                }
                                ok_column_count += 1;
                            }
                            if !col_second_check || ok_column_count != n {
                                continue;
                            }
                            ans = map;
                            break 'main;
                        }
                    }
                }
            }
        }
    }
    if n == 4 {
        'main: for l0 in 0..line_candidates[0].len() {
            for l1 in 0..line_candidates[1].len() {
                for l2 in 0..line_candidates[2].len() {
                    for l3 in 0..line_candidates[3].len() {
                        let map = vec![
                            &line_candidates[0][l0],
                            &line_candidates[1][l1],
                            &line_candidates[2][l2],
                            &line_candidates[3][l3],
                        ];
                        let mut col_first_check = true;
                        for i in 0..n {
                            for j in 0..n {
                                if *map[j][i] != c[i] && *map[j][i] != '.' {
                                    col_first_check = false;
                                    break;
                                } else if *map[j][i] == c[i] {
                                    break;
                                }
                            }
                            if !col_first_check {
                                break;
                            }
                        }
                        if !col_first_check {
                            continue;
                        }
                        let mut col_second_check = true;
                        let mut ok_column_count = 0;
                        for i in 0..n {
                            let mut set = HashSet::new();
                            for j in 0..n {
                                if *map[j][i] != '.' {
                                    if set.contains(&map[j][i]) {
                                        col_second_check = false;
                                        break;
                                    } else {
                                        set.insert(map[j][i]);
                                    }
                                }
                            }
                            if !col_second_check || set.len() != 3 {
                                break;
                            }
                            ok_column_count += 1;
                        }
                        if !col_second_check || ok_column_count != n {
                            continue;
                        }
                        ans = map;
                        break 'main;
                    }
                }
            }
        }
    }
    if n == 3 {
        'main: for l0 in 0..line_candidates[0].len() {
            for l1 in 0..line_candidates[1].len() {
                for l2 in 0..line_candidates[2].len() {
                    let map = vec![
                        &line_candidates[0][l0],
                        &line_candidates[1][l1],
                        &line_candidates[2][l2],
                    ];
                    let mut col_first_check = true;
                    for i in 0..n {
                        for j in 0..n {
                            if *map[j][i] != c[i] && *map[j][i] != '.' {
                                col_first_check = false;
                                break;
                            } else if *map[j][i] == c[i] {
                                break;
                            }
                        }
                        if !col_first_check {
                            break;
                        }
                    }
                    if !col_first_check {
                        continue;
                    }
                    let mut col_second_check = true;
                    let mut ok_column_count = 0;
                    for i in 0..n {
                        let mut set = HashSet::new();
                        for j in 0..n {
                            if *map[j][i] != '.' {
                                if set.contains(&map[j][i]) {
                                    col_second_check = false;
                                    break;
                                } else {
                                    set.insert(map[j][i]);
                                }
                            }
                        }
                        if !col_second_check || set.len() != 3 {
                            break;
                        }
                        ok_column_count += 1;
                    }
                    if !col_second_check || ok_column_count != n {
                        continue;
                    }
                    ans = map;
                    break 'main;
                }
            }
        }
    }
    if ans.len() == 0 {
        println!("No");
    } else {
        println!("Yes");
        for i in 0..n {
            for j in 0..n {
                print!("{}", ans[i][j]);
            }
            println!();
        }
    }
}
