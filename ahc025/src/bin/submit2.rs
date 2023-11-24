use proconio::{input, source::line::LineSource};
use std::{
    collections::HashMap,
    io::{self, BufReader, Write},
};

fn exchange(
    div_num: usize,
    start_div: usize,
    divisions: &mut Vec<Vec<usize>>,
    belongings: &mut HashMap<usize, usize>,
    max_query_num: usize,
    d: usize,
    n: usize,
) -> usize {
    let stdin = io::stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
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
            input! {
                from &mut source,
                _: char,
            }
            query_cnt += 1;
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
            // result = compare(&query, &ws);
            input! {
                from &mut source,
                tmp: char
            }
            result = tmp;
            query_cnt += 1;
        }
        match result {
            '<' => {
                if right_cnt > half_div_num {
                    println!("# < ");
                    io::stdout().flush().unwrap();
                    while divisions[(right_start_div + right_pop_cnt) % d].len() <= 1 {
                        right_pop_cnt = (right_pop_cnt + 1) % half_div_num;
                    }
                    let mut pop_index: usize = 0;
                    if query_cnt < max_query_num {
                        let sub_query1 = format!(
                            "1 1 {} {}",
                            divisions[(right_start_div + right_pop_cnt) % d][0],
                            divisions[(right_start_div + right_pop_cnt) % d][1]
                        );
                        println!("{}", sub_query1);
                        io::stdout().flush().unwrap();
                        query_cnt += 1;

                        // let sub_result1 = compare(&sub_query1, &ws);
                        input! {
                            from &mut source,
                            sub_result1: char,
                        }
                        match sub_result1 {
                            '<' => {
                                if query_cnt < max_query_num
                                    && divisions[(right_start_div + right_pop_cnt) % d].len() >= 3
                                {
                                    let sub_query2 = format!(
                                        "1 1 {} {}",
                                        divisions[(right_start_div + right_pop_cnt) % d][0],
                                        divisions[(right_start_div + right_pop_cnt) % d][2]
                                    );
                                    println!("{}", sub_query2);
                                    io::stdout().flush().unwrap();

                                    query_cnt += 1;
                                    input! {
                                        from &mut source,
                                        sub_result2: char,
                                    }
                                    // let sub_result2 = compare(&sub_query2, &ws);
                                    match sub_result2 {
                                        '<' => {}
                                        '>' => {
                                            pop_index = 2;
                                        }
                                        '=' => {}
                                        _ => {
                                            panic!();
                                        }
                                    }
                                }
                            }
                            '>' => {
                                pop_index = 1;
                                if query_cnt < max_query_num
                                    && divisions[(right_start_div + right_pop_cnt) % d].len() >= 3
                                {
                                    let sub_query2 = format!(
                                        "1 1 {} {}",
                                        divisions[(right_start_div + right_pop_cnt) % d][1],
                                        divisions[(right_start_div + right_pop_cnt) % d][2]
                                    );
                                    println!("{}", sub_query2);
                                    io::stdout().flush().unwrap();

                                    query_cnt += 1;
                                    input! {
                                        from &mut source,
                                        sub_result2: char,
                                    }
                                    // let sub_result2 = compare(&sub_query2, &ws);
                                    match sub_result2 {
                                        '<' => {}
                                        '>' => {
                                            pop_index = 2;
                                        }
                                        '=' => {}
                                        _ => {
                                            panic!();
                                        }
                                    }
                                }
                            }
                            '=' => {
                                if query_cnt < max_query_num
                                    && divisions[(right_start_div + right_pop_cnt) % d].len() >= 3
                                {
                                    let sub_query2 = format!(
                                        "1 1 {} {}",
                                        divisions[(right_start_div + right_pop_cnt) % d][0],
                                        divisions[(right_start_div + right_pop_cnt) % d][2]
                                    );
                                    println!("{}", sub_query2);
                                    io::stdout().flush().unwrap();

                                    query_cnt += 1;
                                    // let sub_result2 = compare(&sub_query2, &ws);
                                    input! {
                                        from &mut source,
                                        sub_result2: char,
                                    }
                                    match sub_result2 {
                                        '<' => {}
                                        '>' => {
                                            pop_index = 2;
                                        }
                                        '=' => {}
                                        _ => {
                                            panic!();
                                        }
                                    }
                                }
                            }
                            _ => {
                                panic!();
                            }
                        }
                    }

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
                    println!("# > ");
                    io::stdout().flush().unwrap();

                    while divisions[(left_start_div + left_pop_cnt) % d].len() <= 1 {
                        left_pop_cnt = (left_pop_cnt + 1) % half_div_num;
                    }

                    let mut pop_index: usize = 0;
                    if query_cnt < max_query_num {
                        let sub_query1 = format!(
                            "1 1 {} {}",
                            divisions[(left_start_div + left_pop_cnt) % d][0],
                            divisions[(left_start_div + left_pop_cnt) % d][1]
                        );
                        println!("{}", sub_query1);
                        io::stdout().flush().unwrap();

                        query_cnt += 1;
                        // let sub_result1 = compare(&sub_query1, &ws);
                        input! {
                            from &mut source,
                            sub_result1: char,
                        }
                        match sub_result1 {
                            '<' => {
                                if query_cnt < max_query_num
                                    && divisions[(left_start_div + left_pop_cnt) % d].len() >= 3
                                {
                                    let sub_query2 = format!(
                                        "1 1 {} {}",
                                        divisions[(left_start_div + left_pop_cnt) % d][0],
                                        divisions[(left_start_div + left_pop_cnt) % d][2]
                                    );
                                    println!("{}", sub_query2);
                                    io::stdout().flush().unwrap();

                                    query_cnt += 1;
                                    // let sub_result2 = compare(&sub_query2, &ws);
                                    input! {
                                        from &mut source,
                                        sub_result2: char,
                                    }
                                    match sub_result2 {
                                        '<' => {}
                                        '>' => {
                                            pop_index = 2;
                                        }
                                        '=' => {}
                                        _ => {
                                            panic!();
                                        }
                                    }
                                }
                            }
                            '>' => {
                                pop_index = 1;
                                if query_cnt < max_query_num
                                    && divisions[(left_start_div + left_pop_cnt) % d].len() >= 3
                                {
                                    let sub_query2 = format!(
                                        "1 1 {} {}",
                                        divisions[(left_start_div + left_pop_cnt) % d][1],
                                        divisions[(left_start_div + left_pop_cnt) % d][2]
                                    );
                                    println!("{}", sub_query2);
                                    io::stdout().flush().unwrap();

                                    query_cnt += 1;
                                    // let sub_result2 = compare(&sub_query2, &ws);
                                    input! {
                                        from &mut source,
                                        sub_result2: char,
                                    }
                                    match sub_result2 {
                                        '<' => {}
                                        '>' => {
                                            pop_index = 2;
                                        }
                                        '=' => {}
                                        _ => {
                                            panic!();
                                        }
                                    }
                                }
                            }
                            '=' => {
                                if query_cnt < max_query_num
                                    && divisions[(left_start_div + left_pop_cnt) % d].len() >= 3
                                {
                                    let sub_query2 = format!(
                                        "1 1 {} {}",
                                        divisions[(left_start_div + left_pop_cnt) % d][0],
                                        divisions[(left_start_div + left_pop_cnt) % d][2]
                                    );
                                    println!("{}", sub_query2);
                                    io::stdout().flush().unwrap();

                                    query_cnt += 1;
                                    // let sub_result2 = compare(&sub_query2, &ws);
                                    input! {
                                        from &mut source,
                                        sub_result2: char,
                                    }
                                    match sub_result2 {
                                        '<' => {}
                                        '>' => {
                                            pop_index = 2;
                                        }
                                        '=' => {}
                                        _ => {
                                            panic!();
                                        }
                                    }
                                }
                            }
                            _ => {
                                panic!();
                            }
                        }
                    }

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
        println!("#c {}", tmp_ans);
        io::stdout().flush().unwrap();

        println!("# {}", tmp_ans);
        io::stdout().flush().unwrap();

        if query_cnt >= max_query_num - 3 {
            break;
        }
    }
    query_cnt
}

fn main() {
    let (n, d, q) = {
        let stdin = io::stdin();
        let mut source = LineSource::new(BufReader::new(stdin.lock()));

        input! {
            from &mut source,
            n: usize,
            d: usize,
            q: usize,
        }
        (n, d, q)
    };

    let mut divisions = vec![vec![]; d];
    let mut belongings = HashMap::new();
    for i in 0..n {
        divisions[i % d].push(i);
        belongings.insert(i, i % d);
    }

    let mut query_cnt = 0;
    let mut div_num = 2;
    let mut start_div = 0;
    loop {
        let group_num = if d % div_num == 0 {
            d / div_num
        } else {
            d / div_num + 1
        };
        println!("# group_num {}", group_num);
        io::stdout().flush().unwrap();
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
            );
            println!("# used query: {}", used_query_cnt);
            io::stdout().flush().unwrap();

            query_cnt += used_query_cnt;
            if query_cnt >= q {
                break;
            }
            start_div = (start_div + div_num) % d;
        }
        // div_num *= 2;
        if query_cnt >= q {
            break;
        }
        if div_num > d {
            div_num = d;
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
