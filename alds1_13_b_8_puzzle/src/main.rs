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

type Board = Vec<usize>;

fn is_target(b: &Board) -> bool {
    b.iter().take(N2 - 1).zip(1..N2).all(|(&i, j)| i == j)
}

fn bfs(input: Board) -> Option<Vec<usize>> {
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();

    // upper, left, right, lower
    const DX: [i32; 4] = [0, -1, 1, 0];
    const DY: [i32; 4] = [1, 0, 0, -1];

    q.push_back((input, vec![]));
    while let Some((board, path)) = q.pop_front() {
        if is_target(&board) {
            return Some(path);
        }
        visited.insert(board.clone());

        // generate possible states
        // swap blank with upper/left/right/lower tile
        assert!(board.iter().filter(|&&i| i == 0).count() > 0);
        let zero_pos = board.iter().position(|&x| x == 0).unwrap();
        let sx = zero_pos % N;
        let sy = zero_pos / N;
        let n = N as i32;
        for i in 0..4 {
            let tx = sx as i32 + DX[i];
            let ty = sy as i32 + DY[i];
            if tx < 0 || tx >= n || ty < 0 || ty >= n {
                continue;
            }
            let tx = tx as usize;
            let ty = ty as usize;

            let mut board_new = board.clone();
            board_new.swap(zero_pos, ty * N + tx);
            if visited.contains(&board_new) {
                continue;
            } else {
                let moved_num = board_new[zero_pos];
                let mut path_new = path.clone();
                path_new.push(moved_num);
                q.push_back((board_new, path_new));
            }
        }
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
        board: [usize; N2],
    }

    solve(board);
}
