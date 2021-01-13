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

// ----------------------------------------------------------------------------------------------------
// main code
// ----------------------------------------------------------------------------------------------------

//cost[i][j] := min cost to calculate M_iM_{i+1}...M_j (i<=j)
struct Solver {
    n: usize,
    dim: Vec<usize>,
    cost: Vec<Vec<Option<u32>>>,
}

impl Solver {
    fn calc_cost(&mut self, i: usize, j: usize) -> u32 {
        if let Some(c) = self.cost[i][j] {
            return c;
        };

        let ret;
        if i >= j {
            ret = 0;
        } else {
            let mut min = std::u32::MAX;
            for k in i..j {
                min = cmp::min(
                    min,
                    self.calc_cost(i, k)
                        + self.calc_cost(k + 1, j)
                        + (self.dim[i] * self.dim[k + 1] * self.dim[j + 1]) as u32,
                )
            }
            ret = min;
        }
        self.cost[i][j] = Some(ret);
        ret
    }
    fn run(&mut self) {
        println!("{}", self.calc_cost(0, self.n - 1))
    }
}

fn main() {
    input! {
        n: usize,
        dim: [[usize; 2]; n]
    };
    let last = dim[n - 1][1];
    let mut dim: Vec<usize> = dim.iter().map(|a| a[0]).collect();
    dim.push(last);

    let mut s = Solver {
        n,
        dim,
        cost: vec![vec![None; n]; n],
    };

    s.run();
}
