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

// returns Some(index) if sorted[index] == key, None otherwise.
fn my_binary_search<T : PartialOrd + Copy>(key: T, sorted: &Vec<T>) -> Option<usize>{
    let mut lower = 0;
    let mut upper = sorted.len();

    while lower < upper {
        let half : usize = (lower + upper) / 2;
        let cur = sorted[half];
        if key == cur {
            return Some(half);
        }
        else if key < cur {
            upper = half;
        }
        else {
            lower = half + 1;
        }
    }
    None
}

fn solve(ss: &Vec<i32>, ts: &Vec<i32>) {
    let count = ts
        .iter()
        .filter(|t| my_binary_search(**t, ss).is_some())
        .collect::<Vec<_>>()
        .len();
    println!("{}", count);
}

fn main() {
    input! {
        n: usize,
        ss: [i32; n],
        q: usize,
        ts: [i32; q],
    }

    solve(&ss, &ts);
}
