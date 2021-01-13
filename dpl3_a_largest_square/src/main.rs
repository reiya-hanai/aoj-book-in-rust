#![allow(unused_macros)]
use std::cmp;
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

struct Solver {
    h: usize,
    w: usize,
    tiles: Vec<Vec<usize>>,
    max_width: Vec<Vec<usize>>,
}
impl Solver {
    fn new(h: usize, w: usize, tiles: Vec<Vec<usize>>) -> Solver {
        Solver {
            h,
            w,
            tiles,
            max_width: vec![vec![0; w]; h],
        }
    }

    fn calc_partial_max_area(&mut self, i: usize, j: usize) {
        // calculate maximum area of square created in upper-left direction from tile (i,j)
        if self.tiles[i][j] == 1 {
            self.max_width[i][j] = 0;
        } else if i == 0 || j == 0 {
            self.max_width[i][j] = 1;
        } else {
            self.max_width[i][j] = cmp::min(
                cmp::min(self.max_width[i - 1][j - 1], self.max_width[i - 1][j]),
                self.max_width[i][j - 1],
            ) + 1;
        }
    }

    fn run(&mut self) {
        for i in 0..self.h {
            for j in 0..self.w {
                self.calc_partial_max_area(i, j);
            }
        }
        let max_width = self.max_width.iter().fold(0, |x, y| {
            cmp::max(x, y.into_iter().max().map(|&x| x).unwrap())
        });
        println!("{}", max_width * max_width);
    }
}
fn main() {
    input! {
        h: usize,
        w: usize,
        tiles: [[usize; w]; h]
    }

    let mut s = Solver::new(h, w, tiles);
    s.run();
}
