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

fn solve_subproblem(
    i: usize,
    target: i32,
    a: &Vec<i32>,
    table: &mut Vec<Vec<Option<bool>>>,
) -> bool {
    if target == 0 {
        return true;
    }
    else if target < 0 {
        return false;
    }
    if i >= a.len() {
        return false;
    }
    let ti = target as usize;
    match table[i][ti] {
        Some(b) => b,
        None => {
            let ret = solve_subproblem(i + 1, target, &a, table)
                || solve_subproblem(i + 1, target - a[i], &a, table);
            table[i][ti] = Some(ret);
            ret
        }
    }
}

fn solve(n: usize, a: &Vec<i32>, _k: usize, m: &Vec<i32>) {
    let m_max = match m.iter().max() {
        Some(&max) => max,
        None => 0,
    } as usize;
    let mut table: Vec<Vec<Option<bool>>> = Vec::new();
    table.resize(n, vec![None; m_max + 1]);
    for mi in m {
        let ret = solve_subproblem(0, *mi as i32, &a, &mut table);
        if ret {
            println!("yes");
        } else {
            println!("no");
        }
    }
}

fn main() {
    input! {
        n: usize,
        als: [i32; n],
        q: usize,
        mls: [i32; q],
    }
    // println!("n: {}", n);
    // println!("as: {:?}", als);
    // println!("q: {}", q);
    // println!("ms: {:?}", mls);
    solve(n, &als, q, &mls);
}
