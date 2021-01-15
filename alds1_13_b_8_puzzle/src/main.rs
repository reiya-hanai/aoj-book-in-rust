#![allow(unused_macros)]
use std::collections::{HashSet, VecDeque};
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

const N: usize = 3;
const N2: usize = N * N;

#[derive(Clone)]
struct Board {
    state: Vec<usize>, // state of board
    path: Vec<usize>,  // sequence of moved numbers from initial state
}

impl Board {
    // upper, left, right, lower
    fn is_solved(&self) -> bool {
        self.state
            .iter()
            .take(N2 - 1)
            .zip(1..N2)
            .all(|(&i, j)| i == j)
    }

    fn generate_possible_boards(&self) -> Vec<Board> {
        assert!(self.state.iter().filter(|&&i| i == 0).count() > 0);
        let zero_pos = self.state.iter().position(|&x| x == 0).unwrap();
        let sx = zero_pos % N;
        let sy = zero_pos / N;
        let n = N as i32;
        // position of tile to move from the blank: upper, left, right, lower
        [(0, 1), (-1, 0), (1, 0), (0, -1)]
            .iter()
            .map(|(dx, dy)| (sx as i32 + dx, sy as i32 + dy))
            .filter(|&(tx, ty)| tx >= 0 && tx < n && ty >= 0 && ty < n)
            .map(|(tx, ty)| {
                let tx = tx as usize;
                let ty = ty as usize;

                let mut state = self.state.clone();
                state.swap(zero_pos, ty * N + tx);
                let mut path = self.path.clone();
                let moved_num = state[zero_pos];
                path.push(moved_num);
                Board { state, path }
            })
            .collect()
    }
}

fn bfs(input: Board) -> Option<Vec<usize>> {
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    q.push_back(input);
    while let Some(board) = q.pop_front() {
        if board.is_solved() {
            return Some(board.path);
        }
        visited.insert(board.state.clone());

        let branches = board.generate_possible_boards();
        branches
            .iter()
            .filter(|&b| !visited.contains(&b.state))
            .for_each(|b| q.push_back(b.clone()));
    }
    None
}

fn solve(board: Board) {
    let ans = bfs(board);
    if let Some(path) = ans {
        println!("{}", path.len());
    }
}

fn main() {
    input! {
        state: [usize; N2],
    }

    let board = Board {
        state,
        path: vec![],
    };
    solve(board);
}
