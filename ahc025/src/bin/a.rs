use proconio::{input, source::line::LineSource};
use std::io::{self, stdin, BufReader, Write};

fn test_io(q: usize) {
    let mut source = LineSource::new(BufReader::new(stdin().lock()));
    for _ in 0..q {
        println!("1 1 0 1");
        io::stdout().flush().unwrap();
        input! {
            from &mut source,
            _: char,
        }
    }
}

fn main() {
    let (n, q, d) = {
        let mut source = LineSource::new(BufReader::new(stdin().lock()));

        input! {
            from &mut source,
            n: usize,
            d: usize,
            q: usize,
        }
        (n, q, d)
    };

    // for _ in 0..q {
    //     println!("1 1 0 1");
    //     io::stdout().flush().unwrap();
    //     input! {
    //         from &mut source,
    //         _: char,
    //     }
    // }
    test_io(q);
    let mut ans = String::new();
    for i in 0..n {
        ans += &(i % d).to_string();
        ans += " ";
    }
    println!("{}", ans);
    io::stdout().flush().unwrap();
}
