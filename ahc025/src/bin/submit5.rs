use proconio::{input, source::line::LineSource};
use std::{
    collections::HashMap,
    io::{self, BufReader, Write},
};

fn search_min(
    num: usize,
    div: usize,
    divisions: &Vec<Vec<usize>>,
    query_cache: &mut HashMap<String, char>,
) -> (usize, usize) {
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
        let cache = query_cache.get(&query);
        let result;
        if cache.is_some() {
            result = *cache.unwrap();
        } else {
            println!("{}", query);
            io::stdout().flush().unwrap();
            used_query_cnt += 1;
            result = {
                input! {
                    from &mut source,
                    tmp: char,
                }
                tmp
            };
            query_cache.insert(query, result);
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

#[derive(PartialEq, Eq)]
enum MinOrMax {
    Min,
    Max,
}

fn search_div(
    num: usize,
    d: usize,
    start_div: usize,
    half_div_num: usize,
    divisions: &mut Vec<Vec<usize>>,
    query_cache: &mut HashMap<String, char>,
    min_or_max: MinOrMax,
) -> (usize, usize) {
    let mut source = LineSource::new(BufReader::new(io::stdin().lock()));
    let mut div_offset = 0;
    let mut used_query_cnt = 0;
    if min_or_max == MinOrMax::Max {
        while divisions[(start_div + div_offset) % d].len() <= 1 {
            div_offset = (div_offset + 1) % half_div_num;
        }
    }
    let mut next_div_offset = (div_offset + 1) % half_div_num;
    if min_or_max == MinOrMax::Max {
        while divisions[(start_div + next_div_offset) % d].len() <= 1 {
            next_div_offset = (next_div_offset + 1) % half_div_num;
        }
    }

    let search_num = if num < half_div_num {
        num
    } else {
        half_div_num
    };
    for _ in 0..search_num {
        let div = (start_div + div_offset) % d;
        let next_div = (start_div + next_div_offset) % d;
        if div == next_div {
            break;
        }
        let mut query = String::new();
        query += &format!("{} {} ", divisions[div].len(), divisions[next_div].len());
        for i in 0..divisions[div].len() {
            query += &format!("{} ", divisions[div][i]);
        }
        for i in 0..divisions[next_div].len() {
            query += &format!("{} ", divisions[next_div][i]);
        }
        let cache = query_cache.get(&query);
        let result;
        if cache.is_some() {
            result = *cache.unwrap();
        } else {
            println!("{}", query);
            io::stdout().flush().unwrap();
            used_query_cnt += 1;
            result = {
                input! {
                    from &mut source,
                    tmp: char,
                }
                tmp
            };
            query_cache.insert(query, result);
        }
        match result {
            '<' => {
                if min_or_max == MinOrMax::Min {
                    next_div_offset = (next_div_offset + 1) % half_div_num;
                } else {
                    div_offset = next_div_offset;
                    next_div_offset = (next_div_offset + 1) % half_div_num;
                    while divisions[(start_div + next_div_offset) % d].len() <= 1 {
                        next_div_offset = (next_div_offset + 1) % half_div_num;
                    }
                }
            }
            '>' => {
                if min_or_max == MinOrMax::Min {
                    div_offset = next_div_offset;
                    next_div_offset = (next_div_offset + 1) % half_div_num;
                } else {
                    next_div_offset = (next_div_offset + 1) % half_div_num;
                    while divisions[(start_div + next_div_offset) % d].len() <= 1 {
                        next_div_offset = (next_div_offset + 1) % half_div_num;
                    }
                }
            }
            '=' => {
                next_div_offset = (next_div_offset + 1) % half_div_num;
                if min_or_max == MinOrMax::Max {
                    while divisions[(start_div + next_div_offset) % d].len() <= 1 {
                        next_div_offset = (next_div_offset + 1) % half_div_num;
                    }
                }
            }
            _ => {
                panic!();
            }
        }
    }
    ((start_div + div_offset) % d, used_query_cnt)
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
    search_min_or_max_div_num: usize,
    mut query_cache: &mut HashMap<String, char>,
) -> usize {
    let mut query_cnt = 0;
    let half_div_num = div_num / 2;
    let left_start_div = start_div;
    let right_start_div = (start_div + half_div_num) % d;
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
    for _ in 0..max_query_num {
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
            let cache = query_cache.get(&query);
            if cache.is_some() {
                result = *cache.unwrap();
            } else {
                println!("{}", query);
                io::stdout().flush().unwrap();
                query_cnt += 1;
                result = {
                    let mut source = LineSource::new(BufReader::new(io::stdin().lock()));
                    input! {
                        from &mut source,
                        tmp: char,
                    }
                    tmp
                };
                query_cache.insert(query, result);
            }
        }
        match result {
            '<' => {
                if right_cnt > half_div_num {
                    println!("# < ");
                    io::stdout().flush().unwrap();
                    let num = if query_cnt + search_min_or_max_div_num <= max_query_num {
                        search_min_or_max_div_num
                    } else {
                        max_query_num - query_cnt
                    };
                    let (right_pop_div, right_used_cnt) = search_div(
                        num,
                        d,
                        right_start_div,
                        half_div_num,
                        divisions,
                        query_cache,
                        MinOrMax::Max,
                    );

                    query_cnt += right_used_cnt;
                    let num = if query_cnt + search_min_or_max_div_num <= max_query_num {
                        search_min_or_max_div_num
                    } else {
                        max_query_num - query_cnt
                    };
                    let (left_push_div, left_used_cnt) = search_div(
                        num,
                        d,
                        left_start_div,
                        half_div_num,
                        divisions,
                        query_cache,
                        MinOrMax::Min,
                    );

                    query_cnt += left_used_cnt;
                    let num = if query_cnt + search_min_num <= max_query_num {
                        search_min_num
                    } else {
                        max_query_num - query_cnt
                    };
                    let (used_query_cnt, pop_index) =
                        search_min(num, right_pop_div, &divisions, &mut query_cache);
                    query_cnt += used_query_cnt;

                    let right_first = divisions[right_pop_div][pop_index as usize];
                    divisions[left_push_div].push(right_first);
                    divisions[right_pop_div].remove(pop_index as usize);
                    belongings.remove(&right_first);
                    belongings.insert(right_first, left_push_div);
                } else {
                    break;
                }
            }
            '>' => {
                if left_cnt > half_div_num {
                    println!("# > ");
                    io::stdout().flush().unwrap();
                    let num = if query_cnt + search_min_or_max_div_num <= max_query_num {
                        search_min_or_max_div_num
                    } else {
                        max_query_num - query_cnt
                    };
                    let (left_pop_div, left_used_cnt) = search_div(
                        num,
                        d,
                        left_start_div,
                        half_div_num,
                        divisions,
                        query_cache,
                        MinOrMax::Max,
                    );
                    query_cnt += left_used_cnt;
                    let num = if query_cnt + search_min_or_max_div_num <= max_query_num {
                        search_min_or_max_div_num
                    } else {
                        max_query_num - query_cnt
                    };
                    let (right_push_div, right_used_cnt) = search_div(
                        num,
                        d,
                        right_start_div,
                        half_div_num,
                        divisions,
                        query_cache,
                        MinOrMax::Min,
                    );

                    query_cnt += right_used_cnt;

                    let num = if query_cnt + search_min_num <= max_query_num {
                        search_min_num
                    } else {
                        max_query_num - query_cnt
                    };
                    let (used_query_cnt, pop_index) =
                        search_min(num, left_pop_div, &divisions, &mut query_cache);
                    query_cnt += used_query_cnt;

                    let left_first = divisions[left_pop_div][pop_index];
                    divisions[right_push_div].push(left_first);
                    divisions[left_pop_div].remove(pop_index);
                    belongings.remove(&left_first);
                    belongings.insert(left_first, right_push_div);
                } else {
                    break;
                }
            }
            '=' => {
                println!("# =");
                io::stdout().flush().unwrap();
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

        if query_cnt + search_min_num + 2 * search_min_or_max_div_num >= max_query_num {
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

    let mut div_num_selected_num_map = HashMap::new();

    let mut query_cache = HashMap::new();

    let max_div_num = if d / 2 > 1 { d / 2 } else { 2 };
    let mut search_min_num = 8;

    let mut divisions = vec![vec![]; d];
    let mut belongings = HashMap::new();
    for i in 0..n {
        divisions[i % d].push(i);
        belongings.insert(i, i % d);
    }

    // let mut current_div = 0;
    let mut query_cnt = 0;
    let mut div_num = 2;
    let search_min_or_max_num = 2;
    let mut start_div = 0;
    let mut div_num_up_cnt = 0;
    let mut used_query_sum_equals_zero = false;
    loop {
        let selected_num = div_num_selected_num_map.get(&div_num);
        match selected_num {
            None => {
                div_num_selected_num_map.insert(div_num, 1);
            }
            Some(num) => {
                div_num_selected_num_map.insert(div_num, num + 1);
            }
        }
        let group_num = if d % div_num == 0 {
            d / div_num
        } else {
            d / div_num + 1
        };
        let mut used_query_sum = 0;
        for _ in 0..group_num {
            let max_query_num = if 2 * n / group_num < q - (query_cnt + used_query_sum) {
                2 * n / group_num
            } else {
                q - (query_cnt + used_query_sum)
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
                search_min_or_max_num,
                &mut query_cache,
            );
            used_query_sum += used_query_cnt;
            start_div = (start_div + div_num) % d;
        }
        query_cnt += used_query_sum;
        if query_cnt >= q {
            break;
        }
        if used_query_sum == 0 || query_cnt >= (div_num_up_cnt + 1) * q / 2 {
            div_num *= 2;
            div_num_up_cnt += 1;
            search_min_num = if search_min_num > div_num_up_cnt * 2 {
                search_min_num - div_num_up_cnt * 2
            } else {
                2
            };
        }
        // if div_num > d {
        //     div_num = d;
        // }
        if div_num > max_div_num {
            if used_query_sum == 0 && used_query_sum_equals_zero {
                break;
            }
            div_num = max_div_num;
        }
        if used_query_sum == 0 {
            used_query_sum_equals_zero = true;
        } else {
            used_query_sum_equals_zero = false;
        }
        // search_min_or_max_num = div_num / 2;
    }

    for _ in 0..(q - query_cnt) {
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

    let mut ans = String::new();
    for i in 0..n {
        let belonging = belongings.get(&i).unwrap();
        ans += &belonging.to_string();
        ans += " ";
    }
    println!("{}", ans);
    io::stdout().flush().unwrap();
}
