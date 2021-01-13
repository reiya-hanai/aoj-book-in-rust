#![allow(unused_macros)]

use std::collections::{LinkedList, VecDeque};
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

fn print_vec<T: ToString>(ts: &Vec<T>, sep: &str) {
    println!(
        "{}",
        ts.iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(sep)
    );
}

// ----------------------------------------------------------------------------------------------------
// main code
// ----------------------------------------------------------------------------------------------------

struct Solver {
    nv: usize,
    adj_ls: Vec<LinkedList<usize>>,
    visited: Vec<bool>,
    idegree: Vec<usize>,
}
impl Solver {
    pub fn new(nv: usize, e: Vec<(usize, usize)>) -> Solver {
        let mut idegree = vec![0; nv];
        let mut adj_ls = vec![LinkedList::new(); nv];
        for &(s, t) in e.iter() {
            idegree[t] += 1;
            adj_ls[s].push_back(t);
        }

        Solver {
            nv,
            adj_ls,
            visited: vec![false; nv],
            idegree,
        }
    }

    pub fn run(&mut self) {
        let mut queue = VecDeque::new();
        let mut result: Vec<usize> = Vec::new();
        (0..self.nv)
            .filter(|&i| self.idegree[i] == 0)
            .for_each(|i| queue.push_back(i));
        while let Some(i) = queue.pop_front() {
            self.visited[i] = true;
            result.push(i);
            let dests = self.adj_ls[i]
                .iter()
                .filter(|&&j| !self.visited[j])
                .collect::<Vec<_>>();
            for &t in dests {
                assert_ne!(self.idegree[t], 0);
                self.idegree[t] -= 1;
                if self.idegree[t] == 0 {
                    queue.push_back(t);
                }
            }
        }
        print_vec(&result, "\n");
    }
}
fn main() {
    input! {
        nv: usize,
        ne: usize,
        edge: [(usize, usize); ne],
    }

    let mut s = Solver::new(nv, edge);
    s.run();
}
