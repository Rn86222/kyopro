use proconio::{fastout, input};
// use itertools::Itertools;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn find_min_2(n: usize, a: usize, b: usize, c: usize, an: &Vec<usize>) -> usize {
    let mut min_2 = std::usize::MAX;
    let mut min_ab = std::usize::MAX;
    let mut min_c = std::usize::MAX;
    let mut ab_selected = -1;
    let mut c_selected = -1;
    let lcm_ab = lcm(a, b);
    // 1つをa,bの倍数に, 1つをcの倍数にする
    for i in 0..n {
        let d = an[i];
        if d % lcm_ab == 0 {
            min_ab = 0;
            ab_selected = i as i32;
        } else {
            if min_ab > lcm_ab - d % lcm_ab {
                min_ab = lcm_ab - d % lcm_ab;
                ab_selected = i as i32;
            }
        }
    }
    for i in 0..n {
        if i as i32 == ab_selected {
            continue;
        }
        let d = an[i];
        if d % c == 0 {
            min_c = 0;
        } else {
            if min_c > c - d % c {
                min_c = c - d % c;
            }
        }
    }
    if min_2 > min_ab + min_c {
        min_2 = min_ab + min_c;
    }

    let mut min_ab = std::usize::MAX;
    let mut min_c = std::usize::MAX;
    for i in 0..n {
        let d = an[i];
        if d % c == 0 {
            min_c = 0;
            c_selected = i as i32;
        } else {
            if min_c > c - d % c {
                min_c = c - d % c;
                c_selected = i as i32;
            }
        }
    }
    for i in 0..n {
        if i as i32 == c_selected {
            continue;
        }
        let d = an[i];
        if d % lcm_ab == 0 {
            min_ab = 0;
        } else {
            if min_ab > lcm_ab - d % lcm_ab {
                min_ab = lcm_ab - d % lcm_ab;
            }
        }
    }
    if min_2 > min_ab + min_c {
        min_2 = min_ab + min_c;
    }
    min_2
}

fn find_min_3(n: usize, a: usize, b: usize, c: usize, an: &Vec<usize>) -> usize {
    let mut selected_a = -1;
    let mut min_a = std::usize::MAX;
    let mut selected_b = -1;
    let mut min_b = std::usize::MAX;
    let mut min_c = std::usize::MAX;
    for i in 0..n {
        let d = an[i];
        if d % a == 0 {
            min_a = 0;
            selected_a = i as i32;
        } else {
            if min_a > a - d % a {
                min_a = a - d % a;
                selected_a = i as i32;
            }
        }
    }
    for i in 0..n {
        if i as i32 == selected_a {
            continue;
        }
        let d = an[i];
        if d % b == 0 {
            min_b = 0;
            selected_b = i as i32;
        } else {
            if min_b > b - d % b {
                min_b = b - d % b;
                selected_b = i as i32;
            }
        }
    }
    for i in 0..n {
        if i as i32 == selected_a || i as i32 == selected_b {
            continue;
        }
        let d = an[i];
        if d % c == 0 {
            min_c = 0;
        } else {
            if min_c > c - d % c {
                min_c = c - d % c;
            }
        }
    }
    min_a + min_b + min_c
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize,
        an: [usize; n],
    }

    let lcm_abc = lcm(lcm(a, b), c);

    let mut min_1 = std::usize::MAX;
    for d in an.clone() {
        if d % lcm_abc == 0 {
            min_1 = 0;
        } else {
            if min_1 > lcm_abc - d % lcm_abc {
                min_1 = lcm_abc - d % lcm_abc;
            }
        }
    }

    let mut min_2 = std::usize::MAX;
    if n >= 2 {
        let min_2_1 = find_min_2(n, a, b, c, &an);
        let min_2_2 = find_min_2(n, a, c, b, &an);
        let min_2_3 = find_min_2(n, b, c, a, &an);

        if min_2_1 > min_2_2 {
            if min_2_2 > min_2_3 {
                min_2 = min_2_3;
            } else {
                min_2 = min_2_2;
            }
        } else {
            if min_2_1 > min_2_3 {
                min_2 = min_2_3;
            } else {
                min_2 = min_2_1;
            }
        }
    }

    let mut min_3 = std::usize::MAX;
    if n >= 3 {
        let min_3s = vec![
            find_min_3(n, a, b, c, &an),
            find_min_3(n, a, c, b, &an),
            find_min_3(n, b, a, c, &an),
            find_min_3(n, b, c, a, &an),
            find_min_3(n, c, b, a, &an),
            find_min_3(n, c, a, b, &an),
        ];
        for min_3_ in min_3s {
            if min_3 > min_3_ {
                min_3 = min_3_;
            }
        }
    }

    let mut min = std::usize::MAX;
    if min > min_1 {
        min = min_1;
    }
    if min > min_2 {
        min = min_2;
    }
    if min > min_3 {
        min = min_3;
    }
    println!("{}", min);
}
