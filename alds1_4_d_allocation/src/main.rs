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

fn solve(n: usize, k: usize, ws: &Vec<usize>) {
    let predicate = |capacity: usize| -> bool {
        let mut partial_sum = 0;
        let mut tracks: Vec<Vec<()>> = Vec::with_capacity(k);
        tracks.resize(k, Vec::with_capacity(n));
        let mut track_id = 0;
        'outer: for i in 0..n {
            while partial_sum + ws[i] > capacity {
                // use next track if possible
                track_id += 1;
                partial_sum = 0;
                if track_id >= k {
                    break 'outer;
                }
            }
            if partial_sum + ws[i] <= capacity {
                tracks[track_id].push(());
                partial_sum += ws[i];
            }
        }
        let len : usize = tracks.iter().map(|t| t.len()).sum();
        len >= ws.len()
    };

    let mut upper = 1_000_000_000;
    let mut lower = 0;

    while lower < upper {
        let half: usize = (lower + upper) / 2;
        if predicate(half) {
            upper = half;
        } else {
            lower = half + 1;
        }
    }
    debug_assert_eq!(upper, lower);
    println!("{}", lower);
}

fn main() {
    input! {
        n: usize,
        k: usize,
        ws: [usize; n],
    }
    // println!("n: {}", n);
    // println!("k: {}", k);
    // println!("ws: {:?}", ws);
    solve(n, k, &ws);
}
