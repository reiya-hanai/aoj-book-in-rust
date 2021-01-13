#![allow(unused_macros)]
#![allow(dead_code)]

use std::cmp::Ordering;
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

#[derive(Clone, Copy, Default, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn from(pair: (f64, f64)) -> Point {
        Point {
            x: pair.0,
            y: pair.1,
        }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Mul<Point> for f64 {
    type Output = Point;
    fn mul(self, other: Point) -> Self::Output {
        Point {
            x: self * other.x,
            y: self * other.y,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let first = self.x.partial_cmp(&other.x);
        if first == Some(Ordering::Equal) {
            self.y.partial_cmp(&other.y)
        } else {
            first
        }
    }
}

fn norm(p: Vector) -> f64 {
    p.x * p.x + p.y * p.y
}

fn abs(p: Vector) -> f64 {
    norm(p).sqrt()
}

fn dot(p: Vector, q: Vector) -> f64 {
    p.x * q.x + p.y * q.y
}

fn cross(p: Vector, q: Vector) -> f64 {
    p.x * q.y - p.y * q.x
}

fn neary_equal(x: f64, y: f64) -> bool {
    let eps = 1e-10;
    if x.abs() < eps && y.abs() < eps {
        true
    } else {
        (x - y).abs() < eps
    }
}

type Vector = Point;

#[derive(Clone, Copy, Default)]
struct Segment {
    p1: Point,
    p2: Point,
}

type Line = Segment;

impl Segment {
    fn from(p1: (f64, f64), p2: (f64, f64)) -> Segment {
        Segment {
            p1: Point::from(p1),
            p2: Point::from(p2),
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct Circle {
    c: Point,
    r: f64,
}

impl Circle {
    fn from(c: (f64, f64), r: f64) -> Circle {
        Circle {
            c: Point::from(c),
            r,
        }
    }
}

type Polygon = Vec<Point>;

fn rotate(v: Vector, theta: f64) -> Vector {
    Vector {
        x: v.x * theta.cos() - v.y * theta.sin(),
        y: v.x * theta.sin() + v.y * theta.cos(),
    }
}

fn cross_point_cc(c1: Circle, c2: Circle) -> Vec<Point> {
    let d = abs(c2.c - c1.c);
    let e = (c2.c - c1.c) * d.recip();
    let e_orth = rotate(e, std::f64::consts::FRAC_PI_2);
    let phi = ((c1.r * c1.r + d * d -c2.r * c2.r) / (2.0 * c1.r * d)).acos();
    let length = (c1.r * phi.sin()).abs();
    let m = c1.c + (c1.r * phi.cos()) * e;
    let p1 = m + length * e_orth;
    let p2 = m - length * e_orth;
    match p1.partial_cmp(&p2) {
        Some(Ordering::Less) => vec![p1, p2],
        _ => vec![p2, p1],
    }
}

fn main() {
    input! {
        p1: (f64, f64),
        r1: f64,
        p2: (f64, f64),
        r2: f64,
    }

    let c1 = Circle::from(p1, r1);
    let c2 = Circle::from(p2, r2);
    let ans = cross_point_cc(c1, c2);
    assert_eq!(ans.len(), 2);
    println!("{:.10} {:.10} {:.10} {:.10}", ans[0].x, ans[0].y, ans[1].x, ans[1].y);

}
