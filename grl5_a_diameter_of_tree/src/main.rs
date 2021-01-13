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
    adj_ls: AdjList<(usize, i32)>,
    distance: Vec<i32>,
    queue: VecDeque<usize>,
}

impl Solver {
    fn new(n: usize, edge: Vec<(usize, usize, i32)>) -> Solver {
        let mut adj_ls = vec![LinkedList::new(); n];
        for (s, t, w) in edge {
            adj_ls[s].push_back((t, w));
            adj_ls[t].push_back((s, w));
        }
        Solver {
            n,
            adj_ls,
            distance: vec![std::i32::MAX; n],
            queue: VecDeque::new(),
        }
    }

    fn reset(&mut self) {
        self.distance.clear();
        self.distance.resize(self.n, std::i32::MAX);
        self.queue.clear();
    }

    fn visit(&mut self, vertex: usize, distance: i32) {
        self.distance[vertex] = distance;
        self.queue.push_back(vertex);
    }

    fn bfs(&mut self, src: usize) {
        self.visit(src, 0);
        while let Some(node) = self.queue.pop_front() {
            let adj_list = self.adj_ls[node]
                .iter()
                .filter(|&&(u, _)| self.distance[u] == std::i32::MAX)
                .map(|&pair| pair)
                .collect::<Vec<_>>();
            adj_list
                .iter()
                .for_each(|&(u, w)| self.visit(u, self.distance[node] + w))
        }
    }

    fn run(&mut self) {
        self.bfs(0);
        let max_elem = self
            .distance
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(i, _)| i)
            .unwrap();
        self.reset();
        self.bfs(max_elem);
        if let Some(diam) = self.distance.iter().max() {
            println!("{}", diam);
        }
    }
}

fn main() {
    input! {
        n: usize,
        e: [(usize, usize, i32); n-1],
    }
    let mut s = Solver::new(n, e);
    s.run();
}
