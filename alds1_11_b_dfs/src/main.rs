#![allow(unused_macros)]
use std::collections::{LinkedList, VecDeque};
use std::io::Read;
use std::str::FromStr;

// ----------------------------------------------------------------------------------------------------
// IO util 2: reader
// ----------------------------------------------------------------------------------------------------

struct Reader<R: Read> {
    reader: R,
}

#[allow(dead_code)]
impl<R: Read> Reader<R> {
    fn new(reader: R) -> Self {
        Reader { reader }
    }

    fn safe_read<T: FromStr>(&mut self) -> Option<T> {
        let token = self
            .reader
            .by_ref()
            .bytes()
            .map(|c| c.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        if token.is_empty() {
            None
        } else {
            token.parse::<T>().ok()
        }
    }

    fn read<T: FromStr>(&mut self) -> T {
        if let Some(s) = self.safe_read() {
            s
        } else {
            eprintln!("Error while reading: input stream empty");
            std::process::exit(1);
        }
    }

    fn read_vec<T: FromStr>(&mut self, len: usize) -> Vec<T> {
        (0..len).map(|_| self.read()).collect()
    }

    fn read_mat<T: FromStr>(&mut self, rows: usize, columns: usize) -> Vec<Vec<T>> {
        (0..rows).map(|_| self.read_vec(columns)).collect()
    }
}

// ----------------------------------------------------------------------------------------------------
// main code
// ----------------------------------------------------------------------------------------------------

type AdjList<T> = Vec<LinkedList<T>>;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Color {
    White,
    Gray,
    Black,
}
use Color::*;

struct Solver {
    n: usize,
    adj_ls: AdjList<usize>,
    timer: usize,
    stack: VecDeque<usize>,
    color: Vec<Color>,
    time_stamp_first: Vec<usize>,
    time_stamp_last: Vec<usize>,
}

impl Solver {
    fn new(n: usize, adj_ls: AdjList<usize>) -> Solver {
        Solver {
            n,
            adj_ls,
            timer: 0,
            stack: VecDeque::new(),
            color: vec![White; n],
            time_stamp_first: vec![0; n],
            time_stamp_last: vec![0; n],
        }
    }

    fn visit(&mut self, v: usize) {
        self.color[v] = Gray;
        self.stack.push_back(v);
        self.timer += 1;
        self.time_stamp_first[v] = self.timer;
    }

    fn dfs(&mut self, vertex: usize) {
        self.visit(vertex);
        while let Some(&v) = self.stack.back() {
            let adj = self.adj_ls[v]
                .iter()
                .filter(|&&adj| self.color[adj] == White)
                .map(|&adj| adj)
                .next();
            if let Some(u) = adj {
                if self.color[u] == White {
                    self.visit(u);
                }
            } else {
                self.stack.pop_back();
                self.color[v] = Black;
                self.timer += 1;
                self.time_stamp_last[v] = self.timer;
            }
        }
    }

    fn run(&mut self) {
        for i in 0..self.n {
            if self.color[i] == White {
                self.dfs(i);
            }
        }

        (0..self.n).for_each(|i| {
            println!(
                "{} {} {}",
                i + 1,
                self.time_stamp_first[i],
                self.time_stamp_last[i]
            )
        })
    }
}

fn main() {
    let cin = std::io::stdin();
    let cin = cin.lock();
    let mut r = Reader::new(cin);

    let n: usize = r.read();
    let mut adj_ls: AdjList<usize> = vec![LinkedList::new(); n];
    for _ in 0..n {
        let id: usize = r.read();
        let k: usize = r.read();
        let ls: LinkedList<usize> = r.read_vec::<usize>(k).iter().map(|v| *v - 1).collect();
        adj_ls[id - 1] = ls;
    }

    let mut s = Solver::new(n, adj_ls);
    s.run();
}
