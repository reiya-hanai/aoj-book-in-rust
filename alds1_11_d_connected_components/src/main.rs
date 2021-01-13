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
// main code
// ----------------------------------------------------------------------------------------------------

type AdjList<T> = Vec<LinkedList<T>>;

struct Solver {
    n: usize,
    rel: Vec<(usize, usize)>,
    query: Vec<(usize, usize)>,
    adj_ls: AdjList<usize>,
    queue: VecDeque<usize>,
    color: Vec<i32>,
    color_id: i32
}

impl Solver {
    fn new(n: usize, rel: Vec<(usize, usize)>, query: Vec<(usize, usize)>) -> Solver {
        Solver {
            n,
            rel,
            query,
            adj_ls: vec![LinkedList::new(); n],
            queue: VecDeque::new(),
            color: vec![-1; n],
            color_id: 0,
        }
    }

    fn build_adj_list(&mut self) {
        for (s, t) in self.rel.clone() {
            self.adj_ls[s].push_back(t);
            self.adj_ls[t].push_back(s);
        }
    }

    fn visit(&mut self, v: usize) {
        assert_eq!(self.color[v], -1);
        self.color[v] = self.color_id;
        self.queue.push_back(v);
    }

    fn bfs(&mut self, src: usize) {
        self.visit(src);
        while let Some(v) = self.queue.pop_front() {
            let adj_list: Vec<usize> = self.adj_ls[v]
                .iter()
                .filter(|&&adj| self.color[adj] == -1)
                .map(|&adj| adj)
                .collect();

            for adj in adj_list {
                self.visit(adj);
            }
        }
    }

    fn run(&mut self) {
        self.build_adj_list();
        for i in 0..self.n {
            if self.color[i] == -1 {
                // assign color to connected components of an uncolored vertex: i
                self.color_id += 1;
                self.bfs(i);
            }
        }
        for (s, t) in self.query.clone() {
            if self.color[s] == self.color[t] {
                println!("yes");
            } else {
                println!("no");
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        rel: [(usize, usize); m],
        q: usize,
        query: [(usize, usize); q],
    };

    let mut s = Solver::new(n, rel, query);
    s.run();
}
