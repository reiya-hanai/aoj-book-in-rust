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

#[derive(Clone, Copy, Default, Debug)]
struct Item {
    v: usize,
    w: usize,
}

struct Solver {
    n: usize,
    w: usize,
    items: Vec<Item>,
    table: Vec<Vec<Option<usize>>>,
}
impl Solver {
    fn new(n: usize, w: usize, items: Vec<(usize, usize)>) -> Solver {
        Solver {
            n,
            w,
            items: items.iter().map(|&(v,w)| Item{v, w}).collect(),
            table: vec![vec![None; w + 1]; n + 1],
        }
    }

    // max value under weight_limit=j, using 0..=i items
    fn recurse(&mut self, i: usize, j: usize) -> usize {
        if let Some(v) = self.table[i][j] {
            return v;
        }
        let ret;
        if j == 0 {
            ret = 0;
        } else if i == 0 {
            ret = 0;
        } else {
            if j >= self.items[i - 1].w {
                ret = cmp::max(
                    self.recurse(i - 1, j),
                    self.recurse(i - 1, j - self.items[i - 1].w ) + self.items[i-1].v,
                );
            } else {
                ret = self.recurse(i - 1, j)
            }
        }
        self.table[i][j] = Some(ret);
        ret
    }

    fn run(&mut self) {
        self.recurse(self.n, self.w);
        println!("{}", self.table[self.n][self.w].unwrap());
    }
}
fn main() {
    input! {
        n: usize,
        w: usize,
        items: [(usize, usize); n],
    }

    let mut s = Solver::new(n, w, items);
    s.run();
}
