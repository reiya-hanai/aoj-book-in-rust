#![allow(unused_macros)]
use std::io::Read;
use std::str::FromStr;

// ----------------------------------------------------------------------------------------------------
// IO util
// ----------------------------------------------------------------------------------------------------

fn print_vec<T : ToString>(ts: &Vec<T>, sep: &str) {
    println!("{}", ts.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(sep));
}

fn print_mat<T : ToString>(table: &Vec<Vec<T>>) {
    let c = table.len();
    (0..c).for_each(|i| print_vec(&table[i], " "))
}

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

type AdjList = Vec<Vec<usize>>;

struct Solver {
    n: usize,
    adj_ls: AdjList,
}

impl Solver {
    fn new(n: usize, adj_ls: AdjList) -> Solver {
        Solver {
            n,
            adj_ls,
        }
    }

    fn convert(&mut self) -> Vec<Vec<usize>>{
        let mut adj_mat = vec![vec![0; self.n]; self.n];
        for i in 0..self.n {
            self.adj_ls[i].iter().for_each(|idx| adj_mat[i][*idx] = 1);
        };
        adj_mat
    }

    fn run(&mut self) {
        let adj_mat = self.convert();
        print_mat(&adj_mat);
    }
}

fn main() {
    let cin = std::io::stdin();
    let cin = cin.lock();
    let mut r = Reader::new(cin);

    let n: usize = r.read();
    let mut adj_ls: AdjList = vec![vec![]; n];
    for _ in 0..n {
        let id: usize = r.read();
        let k: usize = r.read();
        let ls: Vec<usize> = r.read_vec::<usize>(k).iter().map(|v| *v - 1).collect();
        adj_ls[id - 1] = ls;
    }
    // println!("{:?}", adj_ls);

    let mut s = Solver::new(n, adj_ls);
    s.run();
}
