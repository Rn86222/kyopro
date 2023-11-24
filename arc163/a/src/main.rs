macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

use std::{boxed, cmp::Ordering};

fn main() {
    input! {
        T: usize,
        NS: [(usize, chars); T]
    }
    for (n, s) in NS {
        let len = s.len();
        // let fst = s[0];
        // let mut found = false;
        // let mut fst_mark: Vec<bool> = vec![false; len];
        // let mut strings: Vec<String> = vec![];
        // let mut string_first = 1;
        // for (i, c) in s.clone().iter().enumerate() {
        //     if (fst as u32) < (*c as u32) {
        //         found = true;
        //         break;
        //     } else if (fst as u32) == (*c as u32) {
        //         if string_first + 1 < i {
        //             let string = &s.clone()[string_first..=i];
        //             strings.push(string.iter().collect());
        //         }
        //         string_first = i + 1;
        //         fst_mark[i] = true;
        //     }
        // }
        // if found {
        //     println!("Yes");
        //     continue;
        // }
        // let mut before_string = strings.pop();
        // for string in strings {
        //     match before_string.cmp(&Some(string)) {
        //         Ordering::Less => println!("str1 is less than str2"),
        //         Ordering::Greater => println!("str1 is greater than str2"),
        //         Ordering::Equal => println!("str1 and str2 are equal"),
        //     }
        // }
        let mut found = false;
        for i in 1..len {
            match s[..i].cmp(&s[i..]) {
                Ordering::Less => {
                    println!("Yes");
                    found = true;
                    break;
                }
                _ => {
                    continue;
                }
            }
        }
        if !found {
            println!("No");
        }
    }
}
