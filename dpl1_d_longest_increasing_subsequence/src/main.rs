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

struct Solver {
    n: usize,
    a_ls: Vec<usize>,
}
impl Solver {
    fn new(n: usize, a_ls: Vec<usize>) -> Solver {
        Solver {
            n,
            a_ls,
        }
    }

    fn run(&mut self) {
        let mut length = 1;
        let mut tail = vec![std::usize::MAX; self.n + 1];
        tail[0] = self.a_ls[0];

        for i in 1..self.n {
            if self.a_ls[i] > tail[length - 1] {
                tail[length] = self.a_ls[i];
                length += 1;
            } else {
                let pos = tail.iter().position(|&x| x >= self.a_ls[i]);
                if let Some(idx) = pos {
                    tail[idx] = self.a_ls[i];
                }
            }
        }
        println!("{}", length);
    }
}
fn main() {
    input! {
        n: usize,
        a_ls: [usize; n],
    }

    let mut s = Solver::new(n, a_ls);
    s.run();
}
