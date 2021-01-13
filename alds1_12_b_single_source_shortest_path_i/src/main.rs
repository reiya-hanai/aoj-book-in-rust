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
    adj_mat: Vec<Vec<usize>>,
    color: Vec<Color>,
    min_weight: Vec<usize>,
    parent: Vec<Option<usize>>,
}

impl Solver {
    fn new(n: usize, adj_mat: Vec<Vec<usize>>) -> Solver {
        Solver {
            n,
            adj_mat,
            color: vec![White; n],
            min_weight: vec![std::usize::MAX; n],
            parent: vec![None; n],
        }
    }

    fn dijkstra(&mut self, src: usize) {
        self.min_weight[src] = 0;
        self.parent[src] = None;

        loop {
            let mut min_cost = std::usize::MAX;
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
                    if self.color[v] != Black && self.adj_mat[u][v] < std::usize::MAX {
                        if self.min_weight[u] + self.adj_mat[u][v] < self.min_weight[v] {
                            self.min_weight[v] = self.min_weight[u] + self.adj_mat[u][v];
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
        self.dijkstra(0);
        (0..self.n).for_each(|i| println!("{} {}", i, self.min_weight[i]));
    }
}

fn construct_adj_mat<R: Read>(n: usize, reader: &mut Reader<R>) -> Vec<Vec<usize>> {
    let mut mat = vec![vec![std::usize::MAX; n]; n];
    (0..n).for_each(|_| {
        let id: usize = reader.read();
        let k: usize = reader.read();
        (0..k).for_each(|_| {
            let v: usize = reader.read();
            let c: usize = reader.read();
            mat[id][v] = c;
        })
    });
    mat
}

fn main() {
    let cin = std::io::stdin();
    let cin = cin.lock();
    let mut r = Reader::new(cin);

    let n: usize = r.read();
    let adj_mat = construct_adj_mat(n, &mut r);

    let mut s = Solver::new(n, adj_mat);
    s.run();
}