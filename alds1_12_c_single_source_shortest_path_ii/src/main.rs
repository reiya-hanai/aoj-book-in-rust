use std::cmp::Reverse;
use std::collections::{BinaryHeap, LinkedList};
use std::io::Read;
use std::str::FromStr;

// ----------------------------------------------------------------------------------------------------
// IO util 2
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

#[derive(Clone, Copy, PartialEq)]
enum Color {
    White,
    Gray,
    Black,
}
use Color::*;

struct Solver {
    n: usize,
    adj_ls: Vec<LinkedList<(usize, usize)>>,
    color: Vec<Color>,
    min_weight: Vec<usize>,
    parent: Vec<Option<usize>>,
    priority_queue: BinaryHeap<Reverse<(usize, usize)>>,
}

impl Solver {
    fn new(n: usize, adj_ls: Vec<LinkedList<(usize, usize)>>) -> Solver {
        Solver {
            n,
            adj_ls,
            color: vec![White; n],
            min_weight: vec![std::usize::MAX; n],
            parent: vec![None; n],
            priority_queue: BinaryHeap::new(),
        }
    }

    fn dijkstra_with_binary_heap(&mut self, src: usize) {
        self.min_weight[src] = 0;
        self.parent[src] = None;
        self.priority_queue.push(Reverse((0, src)));

        while !self.priority_queue.is_empty() {
            if let Some(Reverse((_, u))) = self.priority_queue.pop() {
                self.color[u] = Black;
                for (v, c) in self.adj_ls[u].iter() {
                    if self.color[*v] != Black {
                        if self.min_weight[u] + c < self.min_weight[*v] {
                            self.min_weight[*v] = self.min_weight[u] + c;
                            self.parent[*v] = Some(u);
                            self.color[*v] = Gray;
                            self.priority_queue.push(Reverse((self.min_weight[*v], *v)));
                        }
                    }
                }
            }
        }
    }

    fn run(&mut self) {
        self.dijkstra_with_binary_heap(0);
        (0..self.n).for_each(|i| println!("{} {}", i, self.min_weight[i]));
    }
}

fn construct_adj_list<R: Read>(
    n: usize,
    reader: &mut Reader<R>,
) -> Vec<LinkedList<(usize, usize)>> {
    let mut ls = vec![LinkedList::new(); n];
    (0..n).for_each(|_| {
        let id: usize = reader.read();
        let k: usize = reader.read();
        (0..k).for_each(|_| {
            let v: usize = reader.read();
            let c: usize = reader.read();
            ls[id].push_back((v, c));
        })
    });
    ls
}

fn main() {
    let cin = std::io::stdin();
    let cin = cin.lock();
    let mut r = Reader::new(cin);

    let n: usize = r.read();
    let adj_ls = construct_adj_list(n, &mut r);

    let mut s = Solver::new(n, adj_ls);
    s.run();
}
