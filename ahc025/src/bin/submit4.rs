use proconio::{input, source::line::LineSource};
use std::{
    collections::HashMap,
    io::{self, BufReader, Write},
};

fn search_min(num: usize, div: usize, divisions: &Vec<Vec<usize>>) -> (usize, usize) {
    let mut source = LineSource::new(BufReader::new(io::stdin().lock()));

    let mut pop_index: usize = 0;
    let mut next_pop_index: usize = 1;
    let mut used_query_cnt = 0;
    let div_len = divisions[div].len();
    for i in 0..num {
        let query = format!(
            "1 1 {} {}",
            divisions[div][pop_index], divisions[div][next_pop_index]
        );
        println!("{}", query);
        io::stdout().flush().unwrap();
        used_query_cnt += 1;
        // let result = compare(&query, ws);
        input! {
            from &mut source,
            result: char,
        }
        match result {
            '<' => {
                next_pop_index = (next_pop_index + 1) % div_len;
            }
            '>' => {
                pop_index = next_pop_index;
                next_pop_index = (next_pop_index + 1) % div_len;
            }
            '=' => {
                next_pop_index = (next_pop_index + 1) % div_len;
            }
            _ => {
                panic!();
            }
        }
        if i + 2 >= div_len {
            break;
        }
    }

    (used_query_cnt, pop_index)
}

fn exchange(
    div_num: usize,
    start_div: usize,
    divisions: &mut Vec<Vec<usize>>,
    belongings: &mut HashMap<usize, usize>,
    max_query_num: usize,
    d: usize,
    n: usize,
    search_min_num: usize,
) -> usize {
    let mut query_cnt = 0;
    let half_div_num = div_num / 2;
    let left_start_div = start_div;
    let right_start_div = (start_div + half_div_num) % d;
    let mut left_push_cnt = 0;
    let mut left_pop_cnt = 0;
    let mut right_push_cnt = 0;
    let mut right_pop_cnt = 0;
    if max_query_num <= 2 {
        for _ in 0..max_query_num {
            println!("1 1 0 1");
            io::stdout().flush().unwrap();
            query_cnt += 1;

            let () = {
                let mut source = LineSource::new(BufReader::new(io::stdin().lock()));
                input! {
                    from &mut source,
                    _: char,
                }
            };
        }
        return query_cnt;
    }
    loop {
        let mut query = String::new();
        let mut left_div = left_start_div;
        let mut left_cnt = 0;
        for _ in 0..half_div_num {
            left_cnt += divisions[left_div].len();
            left_div = (left_div + 1) % d;
        }
        let mut right_div = right_start_div;
        let mut right_cnt = 0;
        for _ in 0..half_div_num {
            right_cnt += divisions[right_div].len();
            right_div = (right_div + 1) % d;
        }
        query += &format!("{} {} ", left_cnt, right_cnt);

        let result;
        if left_cnt == 0 && right_cnt == 0 {
            break;
        } else if left_cnt == 0 {
            result = '<';
        } else if right_cnt == 0 {
            result = '>';
        } else {
            let mut left_div = left_start_div;
            for _ in 0..half_div_num {
                for i in 0..divisions[left_div].len() {
                    query += &format!("{} ", divisions[left_div][i]);
                }
                left_div = (left_div + 1) % d;
            }
            let mut right_div = right_start_div;
            for _ in 0..half_div_num {
                for i in 0..divisions[right_div].len() {
                    query += &format!("{} ", divisions[right_div][i]);
                }
                right_div = (right_div + 1) % d;
            }
            println!("{}", query);
            io::stdout().flush().unwrap();
            let tmp = {
                let mut source = LineSource::new(BufReader::new(io::stdin().lock()));
                input! {
                    from &mut source,
                    result: char,
                }
                result
            };
            result = tmp;
            // result = compare(&query, &ws);
            query_cnt += 1;
        }
        match result {
            '<' => {
                if right_cnt > half_div_num {
                    while divisions[(right_start_div + right_pop_cnt) % d].len() <= 1 {
                        right_pop_cnt = (right_pop_cnt + 1) % half_div_num;
                    }
                    let num = if query_cnt + search_min_num <= max_query_num {
                        search_min_num
                    } else {
                        max_query_num - query_cnt
                    };
                    let (used_query_cnt, pop_index) =
                        search_min(num, (right_start_div + right_pop_cnt) % d, &divisions);
                    query_cnt += used_query_cnt;

                    let right_first =
                        divisions[(right_start_div + right_pop_cnt) % d][pop_index as usize];
                    divisions[(left_start_div + left_push_cnt) % d].push(right_first);
                    divisions[(right_start_div + right_pop_cnt) % d].remove(pop_index as usize);
                    belongings.remove(&right_first);
                    belongings.insert(right_first, (left_start_div + left_push_cnt) % d);
                    left_push_cnt = (left_push_cnt + 1) % half_div_num;
                    right_pop_cnt = (right_pop_cnt + 1) % half_div_num;
                } else {
                    break;
                }
            }
            '>' => {
                if left_cnt > half_div_num {
                    while divisions[(left_start_div + left_pop_cnt) % d].len() <= 1 {
                        left_pop_cnt = (left_pop_cnt + 1) % half_div_num;
                    }

                    let num = if query_cnt + search_min_num <= max_query_num {
                        search_min_num
                    } else {
                        max_query_num - query_cnt
                    };
                    let (used_query_cnt, pop_index) =
                        search_min(num, (left_start_div + left_pop_cnt) % d, &divisions);
                    query_cnt += used_query_cnt;

                    let left_first = divisions[(left_start_div + left_pop_cnt) % d][pop_index];
                    divisions[(right_start_div + right_push_cnt) % d].push(left_first);
                    divisions[(left_start_div + left_pop_cnt) % d].remove(pop_index);
                    belongings.remove(&left_first);
                    belongings.insert(left_first, (right_start_div + right_push_cnt) % d);
                    right_push_cnt = (right_push_cnt + 1) % half_div_num;
                    left_pop_cnt = (left_pop_cnt + 1) % half_div_num;
                } else {
                    break;
                }
            }
            '=' => {
                break;
            }
            _ => panic!(),
        }
        let mut tmp_ans = String::new();
        for i in 0..n {
            let belonging = belongings.get(&i).unwrap();
            tmp_ans += &belonging.to_string();
            tmp_ans += " ";
        }

        if query_cnt + search_min_num >= max_query_num {
            break;
        }
    }
    query_cnt
}

fn main() {
    let (n, d, q) = {
        let mut source = LineSource::new(BufReader::new(io::stdin().lock()));
        input! {
            from &mut source,
            n: usize,
            d: usize,
            q: usize,
        }
        (n, d, q)
    };

    let max_div_num = if d / 2 > 1 { d / 2 } else { 2 };
    let search_min_num = 8;

    let mut divisions = vec![vec![]; d];
    let mut belongings = HashMap::new();
    for i in 0..n {
        divisions[i % d].push(i);
        belongings.insert(i, i % d);
    }

    // let mut current_div = 0;
    let mut query_cnt = 0;
    let mut div_num = 2;
    let mut start_div = 0;
    let mut div_num_up_cnt = 0;
    loop {
        let group_num = if d % div_num == 0 {
            d / div_num
        } else {
            d / div_num + 1
        };
        println!("# group_num {}", group_num);
        for _ in 0..group_num {
            let max_query_num = if 2 * n / group_num < q - query_cnt {
                2 * n / group_num
            } else {
                q - query_cnt
            };
            let used_query_cnt = exchange(
                div_num,
                start_div,
                &mut divisions,
                &mut belongings,
                max_query_num,
                d,
                n,
                search_min_num,
            );
            println!(
                "# used query: {} (max_query_num: {}  remainder: {})",
                used_query_cnt,
                max_query_num,
                q - query_cnt
            );
            query_cnt += used_query_cnt;
            start_div = (start_div + div_num) % d;
        }
        if query_cnt >= q {
            break;
        }
        if query_cnt >= (div_num_up_cnt + 2) * q / 4 {
            div_num *= 2;
            div_num_up_cnt += 1;
        }
        // if div_num > d {
        //     div_num = d;
        // }
        if div_num > max_div_num {
            div_num = max_div_num;
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
