#![allow(unused_macros)]

use std::cmp;
use std::collections::{BTreeSet, LinkedList};
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

#[allow(dead_code)]
fn print_vec<T: ToString>(ts: &Vec<T>, sep: &str) {
    if !ts.is_empty() {
        println!(
            "{}",
            ts.iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join(sep)
        );
    }
}

// ----------------------------------------------------------------------------------------------------
// main code
// ----------------------------------------------------------------------------------------------------

struct Solver {
    nv: usize,
    adj_ls: Vec<LinkedList<usize>>,
    parent: Vec<Option<usize>>,
    prenum: Vec<usize>,
    lowest: Vec<usize>,
    visited: Vec<bool>,
    timer: usize,
}
impl Solver {
    pub fn new(nv: usize, e: Vec<(usize, usize)>) -> Solver {
        let mut adj_ls = vec![LinkedList::new(); nv];
        for &(s, t) in e.iter() {
            adj_ls[s].push_back(t);
            adj_ls[t].push_back(s);
        }

        Solver {
            nv,
            adj_ls,
            parent: vec![None; nv],
            prenum: vec![0; nv],
            lowest: vec![std::usize::MAX; nv],
            visited: vec![false; nv],
            timer: 0,
        }
    }

    fn dfs(&mut self, cur: usize, prev: Option<usize>) {
        {
            self.visited[cur] = true;
            self.timer += 1;
            self.prenum[cur] = self.timer;
            self.lowest[cur] = self.timer;
        }
        for next in self.adj_ls[cur].clone() {
            if !self.visited[next] {
                // DFS-tree edge
                self.parent[next] = Some(cur);
                self.dfs(next, Some(cur));
                self.lowest[cur] = cmp::min(self.lowest[cur], self.lowest[next]);
            } else if prev != Some(next) {
                // back-edge
                self.lowest[cur] = cmp::min(self.lowest[cur], self.prenum[next]);
            }
        }
    }

    pub fn run(&mut self) {
        self.dfs(0, None); // 0: arbitrary root

        let mut art_points = BTreeSet::new();
        // condition i: root
        if self.parent.iter().filter(|&&v| v == Some(0)).count() > 1 {
            art_points.insert(0);
        }
        // condition ii: other vertices
        for i in 0..self.nv {
            if let Some(p) = self.parent[i] {
                if p != 0 && self.prenum[p] <= self.lowest[i] {
                    art_points.insert(p);
                }
            }
        }
        print_vec(&art_points.iter().collect(), "\n");
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
