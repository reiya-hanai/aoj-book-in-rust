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
// IO util: output
// ----------------------------------------------------------------------------------------------------

fn print_vec_with_convertion(ts: &Vec<i32>, sep: &str) {
    println!(
        "{}",
        ts.iter()
            .map(|&t| if t == std::i32::MAX {
                String::from("INF")
            } else {
                t.to_string()
            })
            .collect::<Vec<_>>()
            .join(sep)
    );
}

fn print_mat(table: &Vec<Vec<i32>>) {
    let c = table.len();
    (0..c).for_each(|i| print_vec_with_convertion(&table[i], " "))
}

// ----------------------------------------------------------------------------------------------------
// main code
// ----------------------------------------------------------------------------------------------------

pub fn safe_add(lhs: i32, rhs: i32) -> i32 {
    match (lhs, rhs) {
        (std::i32::MAX, _) | (_, std::i32::MAX) => std::i32::MAX,
        (a, b) => a + b,
    }
}

struct Solver {
    nv: usize,
    apsp: Vec<Vec<i32>>,
}

impl Solver {
    fn new(nv: usize, edge: Vec<(usize, usize, i32)>) -> Solver {
        // apsp(k=0) = adjcency matrix
        let mut adj_mat = vec![vec![std::i32::MAX; nv]; nv];
        for (s, t, d) in edge {
            adj_mat[s][t] = d;
        }
        for i in 0..nv {
            adj_mat[i][i] = 0;
        }
        Solver { nv, apsp: adj_mat }
    }

    fn warshall_floyd(&mut self) {
        for k in 0..self.nv {
            for i in 0..self.nv {
                for j in 0..self.nv {
                    self.apsp[i][j] =
                        cmp::min(self.apsp[i][j], safe_add(self.apsp[i][k], self.apsp[k][j]));
                }
            }
        }
    }

    fn run(&mut self) {
        self.warshall_floyd();
        if (0..self.nv).any(|i| self.apsp[i][i] < 0) {
            println!("NEGATIVE CYCLE");
        } else {
            print_mat(&self.apsp)
        }
    }
}

fn main() {
    input! {
        nv: usize,
        ne: usize,
        e: [(usize, usize, i32); ne],
    }

    let mut s = Solver::new(nv, e);
    s.run();
}
