use proconio::input;
use std::{
    collections::HashMap,
    io::{self, Write},
};

use proconio::source::line::LineSource;
use std::io::{stdin, BufReader};

fn main() {
    let mut source = LineSource::new(BufReader::new(stdin().lock()));

    input! {
        from &mut source,
        n: usize,
        d: usize,
        q: usize,
    }
    let start_time = std::time::Instant::now();

    let mut divisions = vec![vec![]; d];
    let mut belongings = HashMap::new();
    for i in 0..n {
        divisions[i % d].push(i);
        belongings.insert(i, i % d);
    }

    let mut current_div = 0;
    let mut query_cnt = 0;
    loop {
        if start_time.elapsed().as_millis() > 1900 {
            break;
        }
        let mut query = String::new();
        let left_div = current_div;
        query += &format!("{} ", divisions[left_div].len());
        let right_div = (left_div + 1) % d;
        query += &format!("{} ", divisions[right_div].len());
        let result;
        if (divisions[left_div].len() == 0 && divisions[right_div].len() == 0)
            || left_div == right_div
        {
            current_div = (current_div + 2) % d;
            continue;
        } else if divisions[left_div].len() == 0 {
            result = '<';
        } else if divisions[right_div].len() == 0 {
            result = '>';
        } else {
            for i in 0..divisions[left_div].len() {
                query += &format!("{} ", divisions[left_div][i]);
            }
            for i in 0..divisions[right_div].len() {
                query += &format!("{} ", divisions[right_div][i]);
            }
            println!("{}", query);
            io::stdout().flush().unwrap();
            input! {
                from &mut source,
                tmp: char,
            }
            result = tmp;
            query_cnt += 1;
        }
        match result {
            '<' => {
                if divisions[right_div].len() > 1 {
                    println!("# {} < {}", left_div, right_div);
                    let right_first = divisions[right_div][0];
                    divisions[left_div].push(right_first);
                    divisions[right_div].remove(0);
                    belongings.remove(&right_first);
                    belongings.insert(right_first, left_div);
                }
            }
            '>' => {
                if divisions[left_div].len() > 1 {
                    println!("# {} > {}", left_div, right_div);
                    let left_first = divisions[left_div][0];
                    divisions[right_div].push(left_first);
                    divisions[left_div].remove(0);
                    belongings.remove(&left_first);
                    belongings.insert(left_first, right_div);
                }
            }
            '=' => {}
            _ => panic!(),
        }
        current_div = (current_div + (query_cnt % 2 + 1)) % d;
        let mut tmp_ans = String::new();
        tmp_ans += "#c ";
        for i in 0..n {
            let belonging = belongings.get(&i).unwrap();
            tmp_ans += &belonging.to_string();
            tmp_ans += " ";
        }
        println!("{}", tmp_ans);
        if query_cnt == q {
            break;
        }
    }
    for _ in query_cnt..q {
        println!("1 1 0 1");
        io::stdout().flush().unwrap();
        input! {
            _: char,
        }
    }

    let mut ans = String::new();
    for i in 0..n {
        let belonging = belongings.get(&i).unwrap();
        ans += &belonging.to_string();
        ans += " ";
    }
    println!("{}", ans);
    io::stdout().flush().unwrap();
}
