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
// IO util
// ----------------------------------------------------------------------------------------------------
#[allow(dead_code)]
fn print_vec<T: ToString>(v: &Vec<T>, sep: &str) {
    println!(
        "{}",
        v.iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(sep)
    );
}

// ----------------------------------------------------------------------------------------------------
// main code
// ----------------------------------------------------------------------------------------------------

// returns pivot's index after partitioning
// note that right index is included in range.
fn partition(ns: &mut Vec<i32>, left: usize, right: usize) -> usize {
    let pivot = ns[right];
    // println!("[{}]", x);
    let mut i = left;
    for j in left..right {
        if ns[j] <= pivot {
            ns.swap(i, j);
            i += 1;
        }
    };
    ns.swap(i, right);
    i
}

fn print(ns: &Vec<i32>, pivot_idx: usize) {
    println!(
        "{}",
        ns.iter()
            .enumerate()
            .map(|(i, v)| {
                if i == pivot_idx {
                    String::from("[") + &v.to_string() + &String::from("]")
                } else {
                    v.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn solve(ns: &mut Vec<i32>) {
    let p = partition(ns, 0, ns.len() - 1);
    print(ns, p);
}

fn main() {
    input! {
        n: usize,
        ns: [i32; n],
    }
    let mut ns = ns;
    solve(&mut ns);
}
