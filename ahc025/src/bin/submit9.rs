use proconio::{input, source::line::LineSource};
use std::{
    collections::HashMap,
    io::{self, BufReader, Write},
};

fn reverse_query(query: &String) -> String {
    let query_chars = query.split_whitespace().collect::<Vec<&str>>();
    let query_nums = query_chars
        .iter()
        .map(|s| u32::from_str_radix(s, 10).unwrap() as usize)
        .collect::<Vec<usize>>();
    let l_num = query_nums[0];
    let r_num = query_nums[1];
    assert_eq!(l_num + r_num + 2, query_nums.len());
    let mut reversed_query = String::new();
    reversed_query += &format!("{} {} ", r_num, l_num);
    for i in 0..r_num {
        reversed_query += &format!("{} ", query_nums[2 + l_num + i as usize]);
    }
    for i in 0..l_num {
        reversed_query += &format!("{} ", query_nums[2 + i as usize]);
    }
    reversed_query
}

fn search_min_in_div(
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
    io::stdout().flush().unwrap();
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
            query_cache.insert(query.clone(), result);
            query_cache.insert(
                reverse_query(&query),
                if result == '<' {
                    '>'
                } else if result == '>' {
                    '<'
                } else {
                    '='
                },
            );
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

fn search_min_or_max_div(
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
    let mut len_ge_2_exists = false;
    if half_div_num > 3 {
        for i in 0..half_div_num {
            if divisions[(start_div + i) % d].len() > 2 {
                len_ge_2_exists = true;
                break;
            }
        }
    }
    if min_or_max == MinOrMax::Max {
        while divisions[(start_div + div_offset) % d].len() <= if len_ge_2_exists { 2 } else { 1 } {
            div_offset = (div_offset + 1) % half_div_num;
        }
    }
    let mut next_div_offset = (div_offset + 1) % half_div_num;
    if min_or_max == MinOrMax::Max {
        while divisions[(start_div + next_div_offset) % d].len()
            <= if len_ge_2_exists { 2 } else { 1 }
        {
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
            query_cache.insert(query.clone(), result);
            query_cache.insert(
                reverse_query(&query),
                if result == '<' {
                    '>'
                } else if result == '>' {
                    '<'
                } else {
                    '='
                },
            );
        }
        io::stdout().flush().unwrap();
        match result {
            '<' => {
                if min_or_max == MinOrMax::Min {
                    next_div_offset = (next_div_offset + 1) % half_div_num;
                } else {
                    div_offset = next_div_offset;
                    next_div_offset = (next_div_offset + 1) % half_div_num;
                    while divisions[(start_div + next_div_offset) % d].len()
                        <= if len_ge_2_exists { 2 } else { 1 }
                    {
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
                    while divisions[(start_div + next_div_offset) % d].len()
                        <= if len_ge_2_exists { 2 } else { 1 }
                    {
                        next_div_offset = (next_div_offset + 1) % half_div_num;
                    }
                }
            }
            '=' => {
                next_div_offset = (next_div_offset + 1) % half_div_num;
                if min_or_max == MinOrMax::Max {
                    while divisions[(start_div + next_div_offset) % d].len()
                        <= if len_ge_2_exists { 2 } else { 1 }
                    {
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

fn exchange_sub(
    query_cnt: &mut usize,
    max_query_num: usize,
    d: usize,
    belongings: &mut HashMap<usize, usize>,
    divisions: &mut Vec<Vec<usize>>,
    mut query_cache: &mut HashMap<String, char>,
    search_min_or_max_div_num: usize,
    min_start_div: usize,
    max_start_div: usize,
    max_div_num: usize,
    min_div_num: usize,
    search_min_num: usize,
) {
    // if *query_cnt + 2 * search_min_or_max_div_num + search_min_num > max_query_num {
    //     return;
    // }
    let num = if *query_cnt + search_min_or_max_div_num <= max_query_num {
        search_min_or_max_div_num
    } else {
        max_query_num - *query_cnt
    };
    let (pop_div, used_cnt) = search_min_or_max_div(
        num,
        d,
        max_start_div,
        max_div_num,
        divisions,
        query_cache,
        MinOrMax::Max,
    );

    *query_cnt += used_cnt;
    let num = if *query_cnt + search_min_or_max_div_num <= max_query_num {
        search_min_or_max_div_num
    } else {
        max_query_num - *query_cnt
    };
    let (push_div, used_cnt) = search_min_or_max_div(
        num,
        d,
        min_start_div,
        min_div_num,
        divisions,
        query_cache,
        MinOrMax::Min,
    );

    *query_cnt += used_cnt;

    let num = if *query_cnt + search_min_num <= max_query_num {
        search_min_num
    } else {
        max_query_num - *query_cnt
    };
    let (used_query_cnt, pop_index) = search_min_in_div(num, pop_div, &divisions, &mut query_cache);
    *query_cnt += used_query_cnt;

    let pop = divisions[pop_div][pop_index as usize];
    divisions[push_div].push(pop);
    divisions[pop_div].remove(pop_index as usize);
    belongings.remove(&pop);
    belongings.insert(pop, push_div);
}

fn exchange_min_to_max(
    div_num: usize,
    start_div: usize,
    divisions: &mut Vec<Vec<usize>>,
    belongings: &mut HashMap<usize, usize>,
    max_query_num: usize,
    d: usize,
    search_min_or_max_div_num: usize,
    search_min_num: usize,
    query_cache: &mut HashMap<String, char>,
) -> usize {
    let mut used_query_cnt = 0;
    let num = if search_min_or_max_div_num <= max_query_num {
        search_min_or_max_div_num
    } else {
        max_query_num
    };
    let (pop_div, used_query_cnt_pop) = search_min_or_max_div(
        num,
        d,
        start_div,
        div_num,
        divisions,
        query_cache,
        MinOrMax::Max,
    );
    used_query_cnt += used_query_cnt_pop;
    let num = if search_min_or_max_div_num + used_query_cnt <= max_query_num {
        search_min_or_max_div_num
    } else {
        max_query_num - used_query_cnt
    };
    let (push_div, used_query_cnt_push) = search_min_or_max_div(
        num,
        d,
        start_div,
        div_num,
        divisions,
        query_cache,
        MinOrMax::Min,
    );
    used_query_cnt += used_query_cnt_push;

    let num = if search_min_num + used_query_cnt <= max_query_num {
        search_min_num
    } else {
        max_query_num - used_query_cnt
    };
    if num < 3 {
        return used_query_cnt;
    }
    let (used_query_cnt_min, pop_index) = search_min_in_div(num, pop_div, &divisions, query_cache);
    used_query_cnt += used_query_cnt_min;

    let pop = divisions[pop_div][pop_index as usize];
    divisions[push_div].push(pop);
    divisions[pop_div].remove(pop_index as usize);
    belongings.remove(&pop);
    belongings.insert(pop, push_div);

    used_query_cnt
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
    query_cache: &mut HashMap<String, char>,
) -> (usize, bool) {
    let mut end = false;
    let mut before_result = None;
    let mut query_cnt = 0;
    if div_num > 8 {
        query_cnt += exchange_min_to_max(
            div_num,
            start_div,
            divisions,
            belongings,
            max_query_num,
            d,
            2 * search_min_or_max_div_num,
            search_min_num,
            query_cache,
        );
        return (query_cnt, false);
    }
    let left_div_num = if div_num % 2 == 0 {
        div_num / 2
    } else {
        div_num / 2 + 1
    };
    let right_div_num = div_num / 2;
    let left_start_div = start_div;
    let right_start_div = (start_div + left_div_num) % d;
    if max_query_num <= search_min_num {
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
        return (query_cnt, true);
    }
    for _ in 0..max_query_num {
        let mut query = String::new();
        let mut left_div = left_start_div;
        let mut left_cnt = 0;
        for _ in 0..right_div_num {
            left_cnt += divisions[left_div].len();
            left_div = (left_div + 1) % d;
        }
        let mut right_div = right_start_div;
        let mut right_cnt = 0;
        for _ in 0..right_div_num {
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
            for _ in 0..right_div_num {
                for i in 0..divisions[left_div].len() {
                    query += &format!("{} ", divisions[left_div][i]);
                }
                left_div = (left_div + 1) % d;
            }
            let mut right_div = right_start_div;
            for _ in 0..right_div_num {
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
                result = {
                    let mut source = LineSource::new(BufReader::new(io::stdin().lock()));

                    input! {
                        from &mut source,
                        tmp: char,
                    }
                    tmp
                };
                query_cnt += 1;
                query_cache.insert(query.clone(), result);
                query_cache.insert(
                    reverse_query(&query),
                    if result == '<' {
                        '>'
                    } else if result == '>' {
                        '<'
                    } else {
                        '='
                    },
                );
            }
        }
        match result {
            '<' => {
                if before_result.is_some() && before_result.unwrap() == '>' {
                    end = true;
                    break;
                }
                before_result = Some('<');
                if right_cnt > right_div_num {
                    io::stdout().flush().unwrap();
                    exchange_sub(
                        &mut query_cnt,
                        max_query_num,
                        d,
                        belongings,
                        divisions,
                        query_cache,
                        search_min_or_max_div_num,
                        left_start_div,
                        right_start_div,
                        right_div_num,
                        left_div_num,
                        search_min_num,
                    );
                } else {
                    break;
                }
            }
            '>' => {
                if before_result.is_some() && before_result.unwrap() == '<' {
                    end = true;
                    break;
                }
                before_result = Some('>');
                if left_cnt > left_div_num {
                    io::stdout().flush().unwrap();
                    exchange_sub(
                        &mut query_cnt,
                        max_query_num,
                        d,
                        belongings,
                        divisions,
                        query_cache,
                        search_min_or_max_div_num,
                        right_start_div,
                        left_start_div,
                        left_div_num,
                        right_div_num,
                        search_min_num,
                    );
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
        if query_cnt + search_min_num + 2 * search_min_or_max_div_num >= max_query_num {
            break;
        }
    }
    (query_cnt, end)
}

fn tester_sub(n: usize, d: usize, q: usize) {
    let mut div_num_selected_num_map = HashMap::new();

    let mut query_cache = HashMap::new();

    let max_div_num = if d / 2 > 1 { d } else { 2 };
    let mut search_min_num = if 12 < n / d { 12 } else { n / d };
    // let mut search_min_num = 2;

    let mut divisions = vec![vec![]; d];
    let mut belongings = HashMap::new();
    for i in 0..n {
        divisions[i % d].push(i);
        belongings.insert(i, i % d);
    }

    let mut query_cnt = 0;
    let mut div_num = 2;
    let mut search_min_or_max_num = if 4 < q / n { 4 } else { q / n };
    let half_max_div_num = if d % 2 == 0 { d / 2 } else { d / 2 + 1 };
    if search_min_or_max_num < half_max_div_num {
        search_min_or_max_num = half_max_div_num;
    }
    // let search_min_or_max_num = 2;

    let mut start_div = 0;
    let mut div_num_up_cnt = 0;
    let mut used_query_sum_equals_zero = false;

    // let mut same_div_num_cnt = 0;

    let mut max_div_num_up_cnt = 0;
    let mut tmp_div_num = 2;
    while tmp_div_num < max_div_num {
        max_div_num_up_cnt += 1;
        tmp_div_num *= 2;
    }

    'main: loop {
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
            let max_query_num = if n / group_num < q - (query_cnt + used_query_sum) {
                n / group_num
            } else {
                q - (query_cnt + used_query_sum)
            };
            let (used_query_cnt, end) = exchange(
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
            if end && group_num == 1 && (query_cnt + used_query_sum) > q * 7 / 8 {
                query_cnt += used_query_sum;
                break 'main;
            }
            start_div = (start_div + div_num) % d;
        }
        query_cnt += used_query_sum;
        if query_cnt >= q {
            break;
        }
        if used_query_sum == 0 || query_cnt >= (div_num_up_cnt + 1) * q / 4 {
            // if div_num < max_div_num {
            div_num *= 2;
            div_num_up_cnt += 1;
            // same_div_num_cnt = 0;

            if d > 10 || (q as f32 / n as f32) < 2.5 {
                search_min_num = if search_min_num > div_num_up_cnt {
                    search_min_num - div_num_up_cnt
                } else {
                    3
                };
                if search_min_num < 3 {
                    search_min_num = 3;
                }
            }
            // }
        } else {
            // same_div_num_cnt += 1;
        }
        if div_num >= max_div_num {
            if used_query_sum == 0 && used_query_sum_equals_zero {
                break;
            }
            div_num_up_cnt = max_div_num_up_cnt;
            div_num = max_div_num;
        }
        if used_query_sum == 0 {
            used_query_sum_equals_zero = true;
        } else {
            used_query_sum_equals_zero = false;
        }
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
    tester_sub(n, d, q);
}
