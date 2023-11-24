// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        scores: [usize; m],
        ss: [String; n],
    }

    let mut max_score = 0;
    let mut player_scores = vec![];
    let mut unsolved_problem_scoress = vec![];
    for (i, s) in ss.clone().iter().enumerate() {
        let mut score = 0;
        let mut unsolved_problem_scores = vec![];
        for (j, c) in s.chars().enumerate() {
            if c == 'o' {
                score += scores[j];
            } else {
                unsolved_problem_scores.push(scores[j]);
            }
        }
        if max_score < score + i + 1 {
            max_score = score + i + 1;
        }
        player_scores.push(score + i + 1);
        unsolved_problem_scores.sort();
        unsolved_problem_scores.reverse();
        unsolved_problem_scoress.push(unsolved_problem_scores);
    }

    for i in 0..n {
        if player_scores[i] == max_score {
            println!("0");
        } else {
            let mut cnt = 0;
            for unsolved_problem_score in unsolved_problem_scoress[i].clone() {
                player_scores[i] += unsolved_problem_score;
                cnt += 1;
                if player_scores[i] > max_score {
                    println!("{}", cnt);
                    break;
                }
            }
        }
    }
}
