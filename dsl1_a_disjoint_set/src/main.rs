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
    _n: usize,
    _q: usize,
    query: Vec<Vec<usize>>,
    forest: DisjointSet,
}

impl Solver {
    fn new(n: usize, q: usize, query: Vec<Vec<usize>>) -> Solver {
        Solver {
            _n: n,
            _q: q,
            query,
            forest: DisjointSet::new(n),
        }
    }

    fn run(&mut self) {
        self.forest.make_set();
        for v in self.query.iter() {
            assert_eq!(v.len(), 3);
            match v[0] {
                0 => {
                    self.forest.unite(v[1], v[2]);
                }
                1 => {
                    if self.forest.same(v[1], v[2]) {
                        println!("1");
                    } else {
                        println!("0");
                    }
                }
                _ => panic!("unknown query"),
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        query: [[usize; 3]; q],
    }

    let mut s = Solver::new(n, q, query);
    s.run();
}
