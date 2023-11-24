use itertools::Itertools;
use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        S: String
    }
    let mut rate = HashMap::new();
    rate.insert("tourist", 3858);
    rate.insert("ksun48", 3679);
    rate.insert("Benq", 3658);
    rate.insert("Um_nik", 3648);
    rate.insert("apiad", 3638);
    rate.insert("Stonefeang", 3630);
    rate.insert("ecnerwala", 3613);
    rate.insert("mnbvmar", 3555);
    rate.insert("newbiedmy", 3516);
    rate.insert("semiexp", 3481);
    println!("{}", rate.get(S.as_str()).unwrap());
}
