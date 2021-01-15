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

const N: usize = 8;

struct Solver {
    board: Vec<Vec<bool>>,
    row: Vec<i32>,
    column: Vec<bool>,
    dpos: Vec<bool>,
    dneg: Vec<bool>,
}

impl Solver {
    fn new(fixed: Vec<(usize, usize)>) -> Solver {
        let mut board = vec![vec![false; N]; N];
        let mut row = vec![-1; N];
        let mut column = vec![false; N];
        let mut dpos = vec![false; 2 * N - 1];
        let mut dneg = vec![false; 2 * N - 1];
        fixed.into_iter().for_each(|(i, j)| {
            board[i][j] = true;
            row[i] = j as i32;
            column[j] = true;
            dpos[i + j] = true;
            dneg[N - 1 + i - j] = true;
        });
        Solver {
            board,
            row,
            column,
            dpos,
            dneg,
        }
    }

    fn print_tile(&self, i: usize, j: usize) {
        let c: char;
        if self.board[i][j] || self.row[i] == j as i32 {
            c = 'Q'
        } else {
            c = '.'
        }
        print!("{}", c);
    }

    fn print_board(&self) {
        for i in 0..N {
            for j in 0..N {
                self.print_tile(i, j);
            }
            println!("");
        }
    }

    fn recurse(&mut self, i: usize) -> bool {
        if i == N {
            // succeeded to allocate N queens
            return true;
        }
        if self.board[i].iter().any(|&c| c) {
            // row i already has a queen
            self.recurse(i + 1)
        } else {
            // try setting queen to (i, j)
            for j in 0..N {
                if self.column[j] || self.dpos[i + j] || self.dneg[N - 1 + i - j] {
                    continue;
                }
                self.row[i] = j as i32;
                self.column[j] = true;
                self.dpos[i + j] = true;
                self.dneg[N - 1 + i - j] = true;

                if self.recurse(i + 1) {
                    return true;
                } else {
                    self.row[i] = -1;
                    self.column[j] = false;
                    self.dpos[i + j] = false;
                    self.dneg[N - 1 + i - j] = false;
                }
            }
            false
        }
    }

    fn run(&mut self) {
        self.recurse(0);
        self.print_board();
    }
}

fn main() {
    input! {
        k: usize,
        fixed: [(usize, usize); k],
    }

    let mut s = Solver::new(fixed);
    s.run();
}
