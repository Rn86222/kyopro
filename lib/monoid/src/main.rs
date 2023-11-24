mod monoid;
use monoid::*;

fn main() {
    let a = Integer::new(3);
    let b = Integer::new(4);
    let e = Integer::e();
    let c = a * b;
    let d = a * e;

    println!("{} * {} = {}", a, b, c);
    println!("{} * {} = {}", a, e, d);

    let af = Float::Float(2.5);
    let bf = Float::Float(3.0);
    let cf = af * bf;

    println!("{} * {} = {}", af, bf, cf);
}
