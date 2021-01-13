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

struct Solver {
    n: usize,
    adj_ls: AdjList<usize>,
    queue: VecDeque<usize>,
    distance: Vec<i32>,
}

impl Solver {
    fn new(n: usize, adj_ls: AdjList<usize>) -> Solver {
        Solver {
            n,
            adj_ls,
            queue: VecDeque::new(),
            distance: vec![-1; n],
        }
    }

    fn visit(&mut self, vertex: usize, distance: i32) {
        self.distance[vertex] = distance;
        self.queue.push_back(vertex);
    }

    fn bfs(&mut self, vertex: usize) {
        self.visit(vertex, 0);
        while let Some(v) = self.queue.pop_front() {
            let adj_list: Vec<usize> = self.adj_ls[v]
                .iter()
                .filter(|&&adj| self.distance[adj] == -1)
                .map(|&adj| adj)
                .collect();
            
            let d = self.distance[v] + 1;

            adj_list.into_iter().for_each(|adj| self.visit(adj, d));
        }
    }

    fn run(&mut self) {
        self.bfs(0);

        (0..self.n).for_each(|i| {
            println!(
                "{} {}",
                i + 1,
                self.distance[i],
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
