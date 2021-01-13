#![allow(unused_macros)]
use std::cmp;
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

struct Solver {
    n: usize,
    m: usize,
    c: Vec<usize>,
    table: Vec<Vec<Option<usize>>>,
}
impl Solver {
    fn new(n: usize, m: usize, c: Vec<usize>) -> Solver {
        Solver {
            n,
            m,
            c,
            table: vec![vec![None; n + 1]; m + 1],
        }
    }

    // pay j using 0..=i coins
    fn recurse(&mut self, i: usize, j: usize) -> usize {
        if let Some(v) = self.table[i][j] {
            return v;
        }
        let ret;
        if j == 0 {
            ret = 0;
        } else if i == 0 {
            ret = std::usize::MAX;
        } else {
            if j >= self.c[i - 1] {
                ret = cmp::min(
                    self.recurse(i - 1, j),
                    self.recurse(i, j - self.c[i - 1]) + 1,
                );
            } else {
                ret = self.recurse(i - 1, j)
            }
        }
        self.table[i][j] = Some(ret);
        ret
    }

    fn run(&mut self) {
        self.recurse(self.m, self.n);
        println!("{}", self.table[self.m][self.n].unwrap());
    }
}
fn main() {
    input! {
        n: usize,
        m: usize,
        c: [usize; m],
    }

    let mut s = Solver::new(n, m, c);
    s.run();
}
