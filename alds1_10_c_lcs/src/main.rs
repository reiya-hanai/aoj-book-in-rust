use std::cmp;
use std::str::FromStr;
// ----------------------------------------------------------------------------------------------------
// IO util
// ----------------------------------------------------------------------------------------------------
#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

// ----------------------------------------------------------------------------------------------------
// main code
// ----------------------------------------------------------------------------------------------------

struct Solver {
    xs: Vec<char>,
    ys: Vec<char>,
    table: Vec<Vec<Option<u16>>>,
}

impl Solver {
    fn with(xs: Vec<char>, ys: Vec<char>) -> Self {
        Self {
            xs,
            ys,
            table: vec![vec![None; 1001]; 1001],
        }
    }

    fn lcs(&mut self, i: usize, j: usize) -> u16 {
        if let Some(v) = self.table[i][j] {
            return v;
        };
        let ret = match (i, j) {
            (_, 0) | (0, _) => 0,
            (i, j) if self.xs[i-1] == self.ys[j-1] => self.lcs(i - 1, j - 1) + 1,
            (i, j) => cmp::max(self.lcs(i - 1, j), self.lcs(i, j - 1)),
        };
        self.table[i][j] = Some(ret);
        ret
    }

    fn run(&mut self) {
        println!("{}", self.lcs(self.xs.len(), self.ys.len()));
    }
}

fn main() {
    let q: usize = read();
    (0..q).for_each(|_| {
        let xs: String = read();
        let ys: String = read();
        let mut s = Solver::with(xs.chars().collect(), ys.chars().collect());
        s.run();
    })
}
