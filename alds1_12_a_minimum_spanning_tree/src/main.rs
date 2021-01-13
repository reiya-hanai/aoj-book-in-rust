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

#[derive(Clone, Copy, PartialEq)]
enum Color {
    White,
    Gray,
    Black,
}
use Color::*;

struct Solver {
    n: usize,
    adj_mat: Vec<Vec<i32>>,
    color: Vec<Color>,
    min_weight: Vec<i32>,
    parent: Vec<Option<usize>>,
}

impl Solver {
    fn new(n: usize, adj_mat: Vec<Vec<i32>>) -> Solver {
        Solver {
            n,
            adj_mat,
            color: vec![White; n],
            min_weight: vec![std::i32::MAX; n],
            parent: vec![None; n],
        }
    }

    fn prim(&mut self) {
        // take an arbitrary vertex as root
        self.min_weight[0] = 0;
        self.parent[0] = None;

        loop {
            let mut min_cost = std::i32::MAX;
            let mut u = None;
            for i in 0..self.n {
                if self.color[i] != Black && self.min_weight[i] < min_cost {
                    min_cost = self.min_weight[i];
                    u = Some(i);
                }
            }

            if let Some(u) = u {
                self.color[u] = Black;
                for v in 0..self.n {
                    if self.color[v] != Black && self.adj_mat[u][v] >= 0 {
                        if self.adj_mat[u][v] < self.min_weight[v] {
                            self.min_weight[v] = self.adj_mat[u][v];
                            self.parent[v] = Some(u);
                            self.color[v] = Gray;
                        }
                    }
                }
            } else {
                // done
                break;
            }

            
        }
    }

    fn run(&mut self) {
        self.prim();
        println!("{}", self.min_weight.iter().sum::<i32>())
    }
}

fn main() {
    input! {
        n: usize,
        adj_mat: [[i32; n]; n],
    };

    let mut s = Solver::new(n, adj_mat);
    s.run();
}
