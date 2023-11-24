mod mod_int;
use mod_int::*;

type Mint = ModInt<998244353>;

fn main() {
    let x = Mint::new(10000000);
    let y = Mint::new(124993);
    let z = x * y;
    println!("{}", z);
    let n = 10;
    let k = 3;
    let com = ModCom::<998244353>::new(n + 1);
    let ans = com.com(n, k);
    println!("{}", ans);
}
