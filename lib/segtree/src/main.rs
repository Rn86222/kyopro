mod segtree;
use proconio::input;
use segtree::*;

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn reverse(string: Vec<usize>) -> Vec<usize> {
    let reversed_string: Vec<usize> = string.iter().map(|c| (*c + 1) % 2).collect();
    reversed_string
}

fn concat(s1: Vec<usize>, s2: Vec<usize>) -> Vec<usize> {
    let mut cat = s1.clone();
    for c in s2 {
        cat.push(c);
    }
    cat
}

fn main() {
    input! {}
}
