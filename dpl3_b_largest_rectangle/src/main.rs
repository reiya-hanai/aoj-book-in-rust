#![allow(unused_macros)]
use std::cmp;
use std::collections::VecDeque;
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
    histogram: Vec<Vec<usize>>,
}
impl Solver {
    fn new(h: usize, w: usize, tiles: Vec<Vec<usize>>) -> Solver {
        Solver {
            h,
            w,
            tiles,
            histogram: vec![vec![0; w + 1]; h + 1],
        }
    }

    // preprocess: make each cell save the number of subsequent squares in upper direction (include itself)
    // as a result, each row can be regarded as a histogram
    fn create_histograms(&mut self) {
        for i in 0..self.h {
            for j in 0..self.w {
                if self.tiles[i][j] == 1 {
                    self.histogram[i][j] = 0;
                } else if i == 0 {
                    self.histogram[i][j] = 1;
                } else {
                    self.histogram[i][j] = self.histogram[i - 1][j] + 1;
                }
            }
        }
    }

    // regarding row i as a histogram, return max area of rectangle included in the histogram
    fn solve_subproblem(&self, i: usize) -> usize {
        struct Rectangle {
            height: usize,
            pos: usize,
        }
        let mut stack: VecDeque<Rectangle> = VecDeque::new();
        let mut max_area = 0;
        for j in 0..self.w + 1 {
            let h_cur = self.histogram[i][j];
            if let Some(rect) = stack.back() {
                if h_cur > rect.height {
                    stack.push_back(Rectangle {
                        height: h_cur,
                        pos: j,
                    })
                } else if h_cur == rect.height {
                    // nop
                } else {
                    let mut pre_pos = j;
                    while !stack.is_empty() && stack.back().unwrap().height >= h_cur {
                        if let Some(pre) = stack.pop_back() {
                            let area = pre.height * (j - pre.pos);
                            max_area = cmp::max(max_area, area);
                            pre_pos = pre.pos;
                        }
                    }
                    stack.push_back(Rectangle {
                        height: h_cur,
                        pos: pre_pos,
                    })
                }
            } else {
                // stack is empty
                stack.push_back(Rectangle {
                    height: h_cur,
                    pos: j,
                });
            }
        }
        assert!(!stack.is_empty() && stack.len() == 1 && stack.back().unwrap().height == 0);
        max_area
    }

    fn run(&mut self) {
        self.create_histograms();
        let mut max_area = 0;
        (0..(self.h + 1)).for_each(|i| max_area = cmp::max(max_area, self.solve_subproblem(i)));
        println!("{}", max_area);
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
