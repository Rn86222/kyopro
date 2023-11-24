// use std::collections::HashMap;

// use itertools::Itertools;
use proconio::{fastout, input};
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        bss: [[usize; n / m]; m]
    }
    let start_time = std::time::Instant::now();
    let mut ans = vec![];
    let mut max_score = 0;
    let mut max_l = 3;
    while start_time.elapsed().as_millis() < 1900 && max_l < 30 {
        max_l += 1;
        let mut current_ans = vec![];
        let mut score = 10000;
        let seed: [u8; 32] = [1; 32];
        let mut rng: StdRng = SeedableRng::from_seed(seed);

        let mut tmp = vec![vec![]];
        for i in 0..m {
            tmp.push(bss[i].clone());
        }
        let mut bss = tmp;
        let mut id_to_mountain = vec![(0, 0); n + 1];
        for i in 1..=m {
            for j in 0..n / m {
                id_to_mountain[bss[i][j]] = (i, j + 1);
            }
        }
        // id_to_mountain[id] = (i-th(1-indexed) mountain, height_position(1-indexed));
        for id in 1..=n {
            let (montain, height) = id_to_mountain[id];
            let mountain_height = bss[montain].len();
            if height == mountain_height {
                // println!("{} {}", id, 0);
                current_ans.push(format!("{} {}", id, 0));
                bss[montain].remove(height - 1);
                continue;
            }
            let k = mountain_height - height;
            let mut division_num;
            if k < 5 {
                division_num = 1;
            } else if k <= 20 {
                division_num = 2;
            } else {
                division_num = 4;
            }

            let mut flags = vec![false; m + 1];
            let mut candidates = vec![];
            for l in 0..max_l {
                if id + l > n {
                    break;
                }
                let (next_mountain, _) = id_to_mountain[id + l];
                flags[next_mountain] = true;
            }
            for to_mountain in 1..m {
                if !flags[to_mountain] {
                    candidates.push(to_mountain);
                }
            }
            let mut to_mountains = vec![];
            if candidates.len() > 0 {
                if division_num > candidates.len() {
                    division_num = candidates.len();
                }
                let mut set = std::collections::HashSet::new();
                for _ in 0..division_num {
                    let mut to_mountain = candidates[rng.gen_range(0..candidates.len())];
                    while set.contains(&to_mountain) {
                        to_mountain = candidates[rng.gen_range(0..candidates.len())];
                    }
                    set.insert(to_mountain);
                    to_mountains.push(to_mountain);
                }
            } else {
                let mut cnt = 0;
                let mut set = std::collections::HashSet::new();
                while cnt < division_num {
                    let to_mountain = rng.gen_range(1..=m);
                    if set.contains(&to_mountain) {
                        continue;
                    }
                    if to_mountain != montain {
                        to_mountains.push(to_mountain);
                        set.insert(to_mountain);
                        cnt += 1;
                    }
                }
            }
            let division_size = k / division_num;
            let mut vs = vec![];
            for i in 0..division_num {
                vs.push(bss[montain][height + i * division_size]);
            }
            for i in 0..division_num {
                // println!(
                //     "{} {}",
                //     vs[division_num - 1 - i],
                //     to_mountains[division_num - 1 - i]
                // );
                current_ans.push(format!(
                    "{} {}",
                    vs[division_num - 1 - i],
                    to_mountains[division_num - 1 - i]
                ));
            }
            let mut moveds = vec![];
            for _ in 0..k {
                let moved = bss[montain].pop().unwrap();
                moveds.push(moved);
            }
            moveds.reverse();
            for ev in 0..k {
                let to_mountain;
                if ev / division_size >= division_num {
                    to_mountain = to_mountains[division_num - 1];
                } else {
                    to_mountain = to_mountains[ev / division_size];
                }
                bss[to_mountain].push(moveds[ev]);
                id_to_mountain[moveds[ev]] = (to_mountain, bss[to_mountain].len());
            }
            bss[montain].remove(height - 1);
            // println!("{} {}", id, 0);
            current_ans.push(format!("{} {}", id, 0));
            score -= k + division_num;
        }
        if score > max_score {
            max_score = score;
            ans = current_ans;
        }
    }
    // println!("{}", max_score);
    for s in ans {
        println!("{}", s);
    }
}
