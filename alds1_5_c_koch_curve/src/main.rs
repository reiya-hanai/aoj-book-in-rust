#![allow(unused_macros)]
use std::f64::consts;
use std::fmt;
use std::ops::{Add, Mul, Sub};

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

#[derive(Clone, Copy)]
struct Vec2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Vec2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.5} {:.5}", self.x, self.y)
    }
}

struct Matrix2D {
    xx: f64,
    xy: f64,
    yx: f64,
    yy: f64,
}

impl Add for Vec2D {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2D {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Vec2D {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vec2D> for Matrix2D {
    type Output = Vec2D;
    fn mul(self, rhs: Vec2D) -> Self::Output {
        Self::Output {
            x: self.xx * rhs.x + self.xy * rhs.y,
            y: self.yx * rhs.x + self.yy * rhs.y,
        }
    }
}

// prints only the first point of line segment to avoid duplicates
fn solve_subproblem(i: usize, x1: Vec2D, x2: Vec2D) -> () {
    if i == 0 {
        println!("{}", x1);
    } else {
        let s = x1 * (2.0 / 3.0) + x2 * (1.0 / 3.0);
        let t = x1 * (1.0 / 3.0) + x2 * (2.0 / 3.0);
        fn rotate(theta: f64) -> Matrix2D {
            Matrix2D {
                xx: theta.cos(),
                xy: -theta.sin(),
                yx: theta.sin(),
                yy: theta.cos(),
            }
        }
        let u = rotate(consts::PI / 3.0) * (t - s) + s;
        solve_subproblem(i - 1, x1, s);
        solve_subproblem(i - 1, s, u);
        solve_subproblem(i - 1, u, t);
        solve_subproblem(i - 1, t, x2);
    }
}

fn solve(n: usize) {
    let x1 = Vec2D { x: 0.0, y: 0.0 };
    let x2 = Vec2D { x: 100.0, y: 0.0 };
    solve_subproblem(n, x1, x2);
    println!("{}", x2);
}

fn main() {
    input! {
        n: usize,
    }
    solve(n);
}
