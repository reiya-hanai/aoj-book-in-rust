#![allow(unused_macros)]
// ----------------------------------------------------------------------------------------------------
// input macro by @tanakh https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
// ----------------------------------------------------------------------------------------------------
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

// ----------------------------------------------------------------------------------------------------
// main code
// ----------------------------------------------------------------------------------------------------

fn eratosthenes_sieve(n: usize) -> Vec<bool> {
    let mut table = vec![true; n + 1];
    table[0] = false;
    table[1] = false;
    let sqrt_n = (n as f64).sqrt().floor() as usize;
    for i in 2..=sqrt_n {
        if table[i] {
            for j in (2 * i..=n).step_by(i) {
                table[j] = false;
            }
        }
    }
    table
}

fn solve(ns: Vec<usize>) {
    assert!(!ns.is_empty());
    let max = ns.iter().max().map(|&x| x).unwrap();
    let is_prime = eratosthenes_sieve(max);
    println!("{}", ns.iter().filter(|&&n| is_prime[n]).count())
}
fn main() {
    input! {
        n: usize,
        ns: [usize; n],
    }

    solve(ns);
}
