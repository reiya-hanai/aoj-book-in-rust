#![allow(unused_macros)]
#![allow(dead_code)]

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

enum CCW {
    CounterClockwise,
    Clockwise,
    OnLineBack,
    OnLineFront,
    OnSegment,
}

impl CCW {
    fn value(&self) -> i32 {
        match self {
            CCW::CounterClockwise => 1,
            CCW::Clockwise => -1,
            CCW::OnLineBack => 2,
            CCW::OnLineFront => -2,
            CCW::OnSegment => 0,
        }
    }
}

fn ccw(p0: Point, p1: Point, p2: Point) -> CCW {
    let v = p1 - p0;
    let w = p2 - p0;
    let eps = 1e-10;
    if cross(v, w) > eps {
        CCW::CounterClockwise
    } else if cross(v, w) < -eps {
        CCW::Clockwise
    } else if dot(v, w) < -eps {
        CCW::OnLineBack
    } else if norm(v) < norm(w) {
        CCW::OnLineFront
    } else {
        CCW::OnSegment
    }
}

fn intersect(s1: Segment, s2: Segment) -> bool {
    ccw(s1.p1, s1.p2, s2.p1).value() * ccw(s1.p1, s1.p2, s2.p2).value() <= 0
        && ccw(s2.p1, s2.p2, s1.p1).value() * ccw(s2.p1, s2.p2, s1.p2).value() <= 0
}

fn distance_pl(p: Point, l: Line) -> f64 {
    let v = l.p2 - l.p1;
    let w = p - l.p1;
    cross(v, w).abs() / abs(v)
}

fn distance_ps(p: Point, s: Segment) -> f64 {
    if dot(s.p2 - s.p1, p - s.p1) < 0.0 {
        abs(p - s.p1)
    }
    else if dot(s.p1 - s.p2, p - s.p2) < 0.0 {
        abs(p - s.p2)
    } else {
        distance_pl(p, s)
    }
}

fn partial_min(x: f64, y: f64) -> f64 {
    // NaN is not considered
    if x < y {
        x
    } else {
        y
    }
}

fn distance_ss(s1: Segment, s2: Segment) -> f64 {
    if intersect(s1, s2) {
        0.0
    } else {
        partial_min(
            partial_min(distance_ps(s1.p1, s2), distance_ps(s1.p2, s2)),
            partial_min(distance_ps(s2.p1, s1), distance_ps(s2.p2, s1)),
        )
    }
}

fn main() {
    input! {
        q: usize,
        query: [[(f64, f64); 4]; q]
    }

    for p in query {
        assert_eq!(p.len(), 4);
        let s1 = Segment::from(p[0], p[1]);
        let s2 = Segment::from(p[2], p[3]);
        println!("{:.10}", distance_ss(s1, s2));
    }
}
