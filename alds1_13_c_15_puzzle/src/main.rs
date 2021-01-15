#![allow(unused_macros)]
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
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

const N: usize = 4;
const N2: usize = N * N;

#[derive(Clone)]
struct Board {
    state: Vec<usize>, // state of board
    cost: usize,
    md: usize,
}

impl Ord for Board {
    fn cmp(&self, other: &Self) -> Ordering {
        let total = self.md + self.cost;
        let total_other = other.md + other.cost;
        total.cmp(&total_other)
    }
}

impl PartialOrd for Board {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Board {}

impl Board {
    // upper, left, right, lower
    fn new(state: Vec<usize>) -> Board {
        let cost =0;
        let mut md = 0;
        for i in 0..N2 {
            md += manhattan_distance(i, state[i]);
        }
        Board { state, cost, md }
    }

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
                let cost = self.cost + 1;

                let moved_num = state[zero_pos];
                let md = self.md + manhattan_distance(sy * N + sx, moved_num)
                    - manhattan_distance(ty * N + tx, moved_num);
                Board { state, cost, md }
            })
            .collect()
    }
}

// Manhattan distance when tile i has number j
fn manhattan_distance(i: usize, j: usize) -> usize {
    if j == 0 {
        return 0;
    }
    let i = i as i32;
    let k = (j - 1) as i32;
    let n = N as i32;
    let ret = (i % n - k % n).abs() + (i / n - k / n).abs();
    ret as usize
}

fn astar(input: Board) -> Option<usize> {
    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();
    pq.push(Reverse(input));
    while let Some(Reverse(board)) = pq.pop() {
        if board.is_solved() {
            return Some(board.cost);
        }
        visited.insert(board.state.clone());

        let branches = board.generate_possible_boards();
        branches
            .iter()
            .filter(|&b| !visited.contains(&b.state))
            .for_each(|b| pq.push(Reverse(b.clone())));
    }
    None
}

fn solve(board: Board) {
    let ans = astar(board);
    if let Some(cost) = ans {
        println!("{}", cost);
    }
}

fn main() {
    input! {
        state: [usize; N2],
    }

    let board = Board::new(state);
    solve(board);
}
