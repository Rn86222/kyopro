use proconio::input;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

fn compare(input: &String, ws: &Vec<usize>) -> char {
    let input_chars = input.split_whitespace().collect::<Vec<&str>>();
    let input_nums = input_chars
        .iter()
        .map(|s| u32::from_str_radix(s, 10).unwrap() as usize)
        .collect::<Vec<usize>>();
    let l_num = input_nums[0];
    let r_num = input_nums[1];
    assert!(l_num > 0);
    assert!(r_num > 0);
    assert_eq!(l_num + r_num + 2, input_nums.len());
    let mut l_w = 0;
    let mut r_w = 0;
    let mut overlap_check = HashSet::new();
    for i in 0..l_num {
        let check = overlap_check.get(&input_nums[2 + i as usize]);
        assert!(check.is_none());
        overlap_check.insert(&input_nums[2 + i as usize]);
        l_w += ws[input_nums[2 + i as usize]];
    }
    for i in l_num..(l_num + r_num) {
        let check = overlap_check.get(&input_nums[2 + i as usize]);
        assert!(check.is_none());
        overlap_check.insert(&input_nums[2 + i as usize]);
        r_w += ws[input_nums[2 + i as usize]];
    }
    io::stdout().flush().unwrap();
    if l_w < r_w {
        '<'
    } else if l_w > r_w {
        '>'
    } else {
        '='
    }
}

fn calc_score(ans: &String, ws: &Vec<usize>, d: usize) -> usize {
    let mut v = 0;
    let ans_chars = ans.split_whitespace().collect::<Vec<&str>>();
    let ans_nums = ans_chars
        .iter()
        .map(|s| u32::from_str_radix(s, 10).unwrap() as usize)
        .collect::<Vec<usize>>();
    let mut division_sums = vec![0; d];
    for i in 0..ans_nums.len() {
        division_sums[ans_nums[i]] += ws[i];
    }
    let mut ave = 0;
    for i in 0..d {
        ave += division_sums[i];
    }
    ave /= d;
    for i in 0..d {
        v += (division_sums[i] as i64 - ave as i64).pow(2);
    }
    v /= d as i64;
    1 + (100. * (v as f64).sqrt()).floor() as usize
}

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
    ws: &Vec<usize>,
    query_cache: &mut HashMap<String, char>,
) -> (usize, usize) {
    println!("# search_min_in_div {} {}", num, div);
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
            println!("# cache found {} {}", query, cache.unwrap());
            result = *cache.unwrap();
        } else {
            println!("{}", query);
            io::stdout().flush().unwrap();
            used_query_cnt += 1;
            result = compare(&query, ws);
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
    ws: &Vec<usize>,
    query_cache: &mut HashMap<String, char>,
    min_or_max: MinOrMax,
) -> (usize, usize) {
    println!(
        "# search_min_or_max_div {} {}",
        num,
        if min_or_max == MinOrMax::Min {
            "min"
        } else {
            "max"
        }
    );
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
            result = compare(&query, ws);
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

fn exchange_as_many_as_possible(
    divisions: &mut Vec<Vec<usize>>,
    belongings: &mut HashMap<usize, usize>,
    pop_div: usize,
    push_div: usize,
    max_query_num: usize,
    ws: &Vec<usize>,
    n: usize,
    query_cache: &mut HashMap<String, char>,
) -> (usize, bool) {
    println!("# exchange_as_many_as_possible {} {}", pop_div, push_div);
    let mut query_cnt = 0;
    loop {
        if query_cnt >= max_query_num {
            return (query_cnt, false);
        }
        // 1つ適当に選んでmaxからminに渡したときそれでも大小関係が変わらなければそのまま進める
        // 変わった場合は元に戻す
        let mut query = String::new();
        let mut pop_division = divisions[pop_div].clone();
        let mut push_division = divisions[push_div].clone();
        if pop_division.len() == 1 {
            return (query_cnt, false);
        }
        let pop = pop_division[0];
        push_division.push(pop);
        pop_division.remove(0);
        query += &format!("{} {} ", pop_division.len(), push_division.len());
        for i in 0..pop_division.len() {
            query += &format!("{} ", pop_division[i]);
        }
        for i in 0..push_division.len() {
            query += &format!("{} ", push_division[i]);
        }
        let cache = query_cache.get(&query);
        let result;
        if cache.is_some() {
            result = *cache.unwrap();
        } else {
            println!("{}", query);
            io::stdout().flush().unwrap();
            query_cnt += 1;
            result = compare(&query, ws);
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
            '>' => {
                divisions[pop_div] = pop_division;
                divisions[push_div] = push_division;
                belongings.remove(&pop);
                belongings.insert(pop, push_div);

                let mut tmp_ans = String::new();
                for i in 0..n {
                    let belonging = belongings.get(&i).unwrap();
                    tmp_ans += &belonging.to_string();
                    tmp_ans += " ";
                }

                println!("#c {}", tmp_ans);
                io::stdout().flush().unwrap();
            }
            '=' => {
                divisions[pop_div] = pop_division;
                divisions[push_div] = push_division;
                belongings.remove(&pop);
                belongings.insert(pop, push_div);

                let mut tmp_ans = String::new();
                for i in 0..n {
                    let belonging = belongings.get(&i).unwrap();
                    tmp_ans += &belonging.to_string();
                    tmp_ans += " ";
                }
                println!("#c {}", tmp_ans);
                io::stdout().flush().unwrap();
                return (query_cnt, true);
            }
            '<' => {
                break;
            }
            _ => {
                panic!();
            }
        }
    }
    return (query_cnt, false);
}

fn exchange_sub(
    query_cnt: &mut usize,
    max_query_num: usize,
    d: usize,
    n: usize,
    ws: &Vec<usize>,
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
        ws,
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
        ws,
        query_cache,
        MinOrMax::Min,
    );

    *query_cnt += used_cnt;

    if pop_div == push_div {
        return;
    }

    let (used_query_cnt_amap, equals) = exchange_as_many_as_possible(
        divisions,
        belongings,
        pop_div,
        push_div,
        max_query_num - *query_cnt,
        ws,
        n,
        query_cache,
    );
    *query_cnt += used_query_cnt_amap;

    if equals {
        return;
    }

    if divisions[pop_div].len() <= 1 {
        return;
    }

    let num = if *query_cnt + search_min_num <= max_query_num {
        search_min_num
    } else {
        max_query_num - *query_cnt
    };
    if num < 3 {
        return;
    }

    let (used_query_cnt, pop_index) =
        search_min_in_div(num, pop_div, &divisions, ws, &mut query_cache);
    *query_cnt += used_query_cnt;

    let pop = divisions[pop_div][pop_index as usize];
    divisions[push_div].push(pop);
    divisions[pop_div].remove(pop_index as usize);
    belongings.remove(&pop);
    belongings.insert(pop, push_div);
}

fn exchange_max_to_min(
    div_num: usize,
    start_div: usize,
    divisions: &mut Vec<Vec<usize>>,
    belongings: &mut HashMap<usize, usize>,
    max_query_num: usize,
    d: usize,
    n: usize,
    ws: &Vec<usize>,
    search_min_or_max_div_num: usize,
    search_min_num: usize,
    query_cache: &mut HashMap<String, char>,
) -> usize {
    println!(
        "# exchange_max_to_min {} {}",
        search_min_or_max_div_num, search_min_num
    );
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
        ws,
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
        ws,
        query_cache,
        MinOrMax::Min,
    );
    used_query_cnt += used_query_cnt_push;

    if pop_div == push_div {
        return used_query_cnt;
    }

    let (used_query_cnt_amap, equals) = exchange_as_many_as_possible(
        divisions,
        belongings,
        pop_div,
        push_div,
        max_query_num - used_query_cnt,
        ws,
        n,
        query_cache,
    );
    println!("# {:?}", divisions);

    used_query_cnt += used_query_cnt_amap;
    if equals {
        return used_query_cnt;
    }
    let num = if search_min_num + used_query_cnt <= max_query_num {
        search_min_num
    } else {
        return used_query_cnt;
        // max_query_num - used_query_cnt
    };

    if divisions[pop_div].len() <= 1 {
        return used_query_cnt;
    }
    // if num < 3 {
    //     return used_query_cnt;
    // }
    let (used_query_cnt_min, pop_index) =
        search_min_in_div(num, pop_div, &divisions, ws, query_cache);
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
    ws: &Vec<usize>,
    search_min_num: usize,
    search_min_or_max_div_num: usize,
    query_cache: &mut HashMap<String, char>,
) -> (usize, bool) {
    let mut query_cnt = 0;
    if div_num > 8 || d == 3 || div_num == d {
        query_cnt += exchange_max_to_min(
            div_num,
            start_div,
            divisions,
            belongings,
            max_query_num,
            d,
            n,
            ws,
            2 * search_min_or_max_div_num,
            search_min_num,
            query_cache,
        );

        let mut tmp_ans = String::new();
        for i in 0..n {
            let belonging = belongings.get(&i).unwrap();
            tmp_ans += &belonging.to_string();
            tmp_ans += " ";
        }
        println!("#c {}", tmp_ans);
        io::stdout().flush().unwrap();
        return (query_cnt, false);
    }

    let mut end = false;
    let mut before_result = None;
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
            println!(
                "# left and right are zero {} {} {}",
                div_num, left_start_div, right_start_div
            );
            io::stdout().flush().unwrap();
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
                result = compare(&query, &ws);
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
                    println!("# < and >");
                    io::stdout().flush().unwrap();
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
                        n,
                        ws,
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
                    println!("# < and >");
                    io::stdout().flush().unwrap();
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
                        n,
                        ws,
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
        if query_cnt + search_min_num + 2 * search_min_or_max_div_num >= max_query_num {
            break;
        }
    }
    (query_cnt, end)
}

fn tester(index: usize) -> (usize, usize, usize, usize) {
    let (n, d, q, ws) = {
        let mut n = 0;
        let mut d = 0;
        let mut q = 0;
        let mut ws = vec![];
        match File::open(&format!("./in/{:>04}.txt", index)) {
            Err(e) => {
                println!("Failed in opening file ({}).", e);
                panic!();
            }
            Ok(file) => {
                let reader = BufReader::new(file);
                for (i, line) in reader.lines().enumerate() {
                    let input = line.unwrap();
                    let input_chars = input.split_whitespace().collect::<Vec<&str>>();
                    if i == 0 {
                        n = input_chars[0].parse::<usize>().unwrap();
                        d = input_chars[1].parse::<usize>().unwrap();
                        q = input_chars[2].parse::<usize>().unwrap();
                        println!("# {} {} {}", n, d, q);
                    } else if i == 1 {
                        ws = input_chars
                            .iter()
                            .map(|s| s.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>();
                        println!("# {:?}", ws);
                    }
                }
                (n, d, q, ws)
            }
        }
    };
    (tester_sub(n, d, q, &ws), n, d, q)
}

fn tester_sub(n: usize, d: usize, q: usize, ws: &Vec<usize>) -> usize {
    let mut div_num_selected_num_map = HashMap::new();

    let mut query_cache = HashMap::new();

    let max_div_num = if d / 2 > 1 { d } else { 2 };
    let mut search_min_num = if 12 < n / d { 12 } else { n / d };
    if d == 2 {
        search_min_num = n / 2;
    }

    let mut divisions = vec![vec![]; d];
    let mut belongings = HashMap::new();
    for i in 0..n {
        divisions[i % d].push(i);
        belongings.insert(i, i % d);
    }

    let mut tmp_ans = String::new();
    for i in 0..n {
        let belonging = belongings.get(&i).unwrap();
        tmp_ans += &belonging.to_string();
        tmp_ans += " ";
    }
    println!("#c {}", tmp_ans);
    io::stdout().flush().unwrap();

    let mut query_cnt = 0;
    let mut div_num = if max_div_num < 4 { max_div_num } else { 4 };
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
    let mut tmp_div_num = div_num;
    while tmp_div_num < max_div_num {
        max_div_num_up_cnt += 1;
        tmp_div_num *= 2;
    }

    println!(
        "# search_min_num {} search_min_or_max_num {}",
        search_min_num, search_min_or_max_num
    );

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
        let mut group_num = if d % div_num == 0 {
            d / div_num
        } else {
            d / div_num + 1
        };
        if d == 3 {
            group_num = 1;
            div_num = 3;
        }
        println!("# group_num {}, div_num {}", group_num, div_num);
        println!("# {:?}", divisions);
        io::stdout().flush().unwrap();
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
                &ws,
                search_min_num,
                search_min_or_max_num,
                &mut query_cache,
            );
            println!(
                "# used query: {} (max_query_num: {}  remainder: {})",
                used_query_cnt,
                max_query_num,
                q - query_cnt - used_query_sum - used_query_cnt
            );
            io::stdout().flush().unwrap();
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

            if d > 10 || (d > 4 && (q as f32 / n as f32) < 2.5) {
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

    assert!(query_cnt <= q);

    for _ in 0..(q - query_cnt) {
        println!("1 1 0 1");
        io::stdout().flush().unwrap();
        query_cnt += 1;
    }

    let mut ans = String::new();
    for i in 0..n {
        let belonging = belongings.get(&i).unwrap();
        ans += &belonging.to_string();
        ans += " ";
    }
    println!("{}", ans);
    io::stdout().flush().unwrap();

    let score = calc_score(&ans, &ws, d);
    println!("# score: {}", score);
    io::stdout().flush().unwrap();

    print!("# ");
    for (k, v) in div_num_selected_num_map {
        print!("{}: {} ", k, v);
    }
    println!("");
    io::stdout().flush().unwrap();

    score
}

fn main() {
    let argc = std::env::args().len();
    if argc >= 2 {
        let mut scores = vec![];
        let test_case_num = 1000;
        for i in 0..test_case_num {
            let (score, n, d, q) = tester(i);
            scores.push((i, score, n, d, q));
            // eprint!("{} ", i);
        }
        let mut sum = 0;
        for i in 0..test_case_num {
            sum += scores[i].1;
        }
        scores.sort_by(|a, b| b.1.cmp(&a.1));
        for i in 0..1000 {
            eprintln!(
                "{} {} {} {}",
                scores[i].0,
                scores[i].1,
                scores[i].4 as f32 / scores[i].2 as f32,
                scores[i].3
            );
        }
        eprintln!("# average score: {}", sum / test_case_num);
        return;
    }
    input! {
        n: usize,
        d: usize,
        q: usize,
        ws: [usize; n]
    }
    tester_sub(n, d, q, &ws);
}
