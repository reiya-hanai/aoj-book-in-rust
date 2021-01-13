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

fn solve(n: usize, keys: &Vec<i32>) {
    let valid_key = |id: i32| -> Option<i32> {
        if 0 <= id && id < n as i32 {
            let id = id as usize;
            Some(keys[id])
        } else {
            None
        }
    };
    for i in 0..n {
        let id = i + 1;
        let key = keys[i];
        let i = i as i32;
        let left = valid_key(2 * i + 1);
        let right = valid_key(2 * i + 2);
        let parent = valid_key(((i - 1) as f64 / 2.0).floor() as i32);
        print!("node {}: key = {}, ", id, key);
        if let Some(parent) = parent {
            print!("parent key = {}, ", parent);
        }
        if let Some(left) = left {
            print!("left key = {}, ", left);
        }
        if let Some(right) = right {
            print!("right key = {}, ", right);
        }
        print!("\n");
    }
}

fn main() {
    input! {
        n: usize,
        ks: [i32; n],
    }
    solve(n, &ks);
}
