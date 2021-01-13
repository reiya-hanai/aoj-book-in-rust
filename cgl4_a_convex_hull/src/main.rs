#![allow(unused_macros)]
#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::VecDeque;
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

const EPS: f64 = 1e-10;

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
    if x.abs() < EPS && y.abs() < EPS {
        true
    } else {
        (x - y).abs() < EPS
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

#[derive(Clone, Default, Debug)]
struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn from(pairs: Vec<(f64, f64)>) -> Polygon {
        Polygon {
            points: pairs.iter().map(|&pair| Point::from(pair)).collect(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum CCW {
    CounterClockwise,
    Clockwise,
    OnLineBack,
    OnLineFront,
    OnSegment,
}

fn ccw(p0: Point, p1: Point, p2: Point) -> CCW {
    let v = p1 - p0;
    let w = p2 - p0;
    if cross(v, w) > 0.0 {
        CCW::CounterClockwise
    } else {
        CCW::Clockwise
    }
}

fn convex_hull(poly: Polygon) -> Polygon {
    // sort points
    let mut points = poly.points;
    points.sort_by(|p1, p2| {
        let first = p1.x.partial_cmp(&p2.x).unwrap();
        if first == Ordering::Equal {
            p1.y.partial_cmp(&p2.y).unwrap()
        } else {
            first
        }
    });
    let poly = Polygon { points };

    let n = poly.points.len();
    if n < 3 {
        return poly;
    }
    // upper part
    let mut upper = VecDeque::new();
    upper.push_back(poly.points[0]);
    upper.push_back(poly.points[1]);
    for i in 2..n {
        let p = poly.points[i];
        loop {
            let m = upper.len();
            if m >= 2 && ccw(upper[m - 2], upper[m - 1], p) != CCW::Clockwise {
                upper.pop_back();
            } else {
                break;
            }
        }
        upper.push_back(p)
    }

    // lower part
    let mut lower = Vec::new();
    lower.push(poly.points[n - 1]);
    lower.push(poly.points[n - 2]);
    for i in (0..n - 2).rev() {
        let p = poly.points[i];
        loop {
            let m = lower.len();
            if m >= 2 && ccw(lower[m - 2], lower[m - 1], p) != CCW::Clockwise {
                lower.pop();
            } else {
                break;
            }
        }
        lower.push(p)
    }

    lower.reverse();
    upper.pop_back();
    upper.pop_front();
    while let Some(p) = upper.pop_back() {
        lower.push(p);
    }

    Polygon {
        points: lower.into_iter().collect::<Vec<_>>(),
    }
}

fn main() {
    input! {
        n: usize,
        pairs: [(f64, f64); n],
    }

    let a = Polygon::from(pairs);
    let mut ans = convex_hull(a);

    // rotate so that first element has smallest y value
    let idx = ans
        .points
        .iter()
        .enumerate()
        .min_by(|(_, p1), (_, p2)| {
            let first = p1.y.partial_cmp(&p2.y);
            if first == Some(Ordering::Equal) {
                p1.x.partial_cmp(&p2.x).unwrap()
            } else {
                first.unwrap()
            }
        })
        .map(|(i, _)| i)
        .unwrap();
    ans.points.rotate_left(idx);

    println!("{}", ans.points.len());
    for p in ans.points {
        println!("{} {}", p.x, p.y);
    }
}
