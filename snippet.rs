// ----------------------------------------------------------------------------------------------------
// IO util: input
// ----------------------------------------------------------------------------------------------------
#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_vec_h<T: FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim()
        .split_whitespace()
        .map(|t| t.parse().ok().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_vec_v<T: FromStr>(size: usize) -> Vec<T> {
    (0..size).map(|_| read()).collect::<Vec<_>>()
}

impl<T> FromStr for Command<T>
where
    T: FromStr,
{
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {}
}

impl<T> ToString for Command<T>
where
    T: ToString,
{
    fn to_string(&self) -> String {}
}

// ----------------------------------------------------------------------------------------------------
// IO util: output
// ----------------------------------------------------------------------------------------------------

fn print_vec<T: ToString>(ts: &Vec<T>, sep: &str) {
    if !ts.is_empty() {
        println!(
            "{}",
            ts.iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join(sep)
        );
    }
}

fn print_mat<T: ToString>(table: &Vec<Vec<T>>) {
    let c = table.len();
    (0..c).for_each(|i| print_vec(&table[i], " "))
}

// ----------------------------------------------------------------------------------------------------
// IO util 2
// ----------------------------------------------------------------------------------------------------
use std::io::Read;
use std::str::FromStr;

struct Reader<R: Read> {
    reader: R,
}

#[allow(dead_code)]
impl<R: Read> Reader<R> {
    fn new(reader: R) -> Self {
        Reader { reader }
    }

    fn safe_read<T: FromStr>(&mut self) -> Option<T> {
        let token = self
            .reader
            .by_ref()
            .bytes()
            .map(|c| c.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        if token.is_empty() {
            None
        } else {
            token.parse::<T>().ok()
        }
    }

    fn read<T: FromStr>(&mut self) -> T {
        if let Some(s) = self.safe_read() {
            s
        } else {
            eprintln!("Error while reading: input stream empty");
            std::process::exit(1);
        }
    }

    fn read_vec<T: FromStr>(&mut self, len: usize) -> Vec<T> {
        (0..len).map(|_| self.read()).collect()
    }

    fn read_mat<T: FromStr>(&mut self, rows: usize, columns: usize) -> Vec<Vec<T>> {
        (0..rows).map(|_| self.read_vec(columns)).collect()
    }
}



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

fn main() {
    let cin = std::io::stdin();
    let cin = cin.lock();
    let mut r = Reader::new(cin);

    let n = r.read();

    let mut s = Solver::new();
    s.run();
}