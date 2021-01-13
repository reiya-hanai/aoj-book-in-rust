#![allow(unused_macros)]

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

struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    fn new(size: usize) -> DisjointSet {
        DisjointSet {
            parent: vec![0; size],
            rank: vec![0; size],
        }
    }

    fn make_set(&mut self) {
        for i in 0..self.parent.len() {
            self.parent[i] = i;
        }
    }

    fn find_set(&mut self, x: usize) -> usize {
        // route compression
        if self.parent[x] != x {
            self.parent[x] = self.find_set(self.parent[x]);
        };
        self.parent[x]
    }

    fn unite(&mut self, x: usize, y: usize) {
        let x = self.find_set(x);
        let y = self.find_set(y);
        if self.rank[x] < self.rank[y] {
            self.parent[x] = y;
        } else {
            self.parent[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find_set(x) == self.find_set(y)
    }
}

struct Solver {
    n: usize,
    e: Vec<(usize, usize, i32)>,
}

impl Solver {
    fn new(n: usize, edge: Vec<(usize, usize, i32)>) -> Solver {
        let mut e = edge.clone();
        e.sort_by(|(_, _, w), (_, _, v)| w.cmp(v));
        Solver { n, e }
    }

    fn run(&mut self) {
        let mut ds = DisjointSet::new(self.n);
        ds.make_set();
        let mut result = Vec::new();
        for &(s, t, w) in self.e.iter() {
            if !ds.same(s, t) {
                ds.unite(s, t);
                result.push(w);
            }
        }
        println!("{}", result.iter().sum::<i32>())
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
