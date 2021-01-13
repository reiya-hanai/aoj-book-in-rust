#![allow(unused_macros)]
#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::mem;
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
    x: i32,
    y: i32,
}

impl Point {
    fn from(pair: (i32, i32)) -> Point {
        Point {
            x: pair.0,
            y: pair.1,
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct Segment {
    p1: Point,
    p2: Point,
}

type Line = Segment;

impl Segment {
    fn from(p1: (i32, i32), p2: (i32, i32)) -> Segment {
        Segment {
            p1: Point::from(p1),
            p2: Point::from(p2),
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
enum EPKind {
    Bottom(i32),
    LR(i32, i32),
    Top(i32),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct EndPoint {
    y: i32,
    kind: EPKind,
}
use EPKind::*;

impl EndPoint {
    fn from(y: i32, kind: EPKind) -> Self {
        Self {
            y,
            kind,
        }
    }
}

impl PartialOrd for EndPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for EndPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        let first = self.y.cmp(&other.y);
        if first == Ordering::Equal {
            self.kind.cmp(&other.kind)
        } else {
            first
        }
    }
}

fn manhattan_intersect(segments: &Vec<Segment>) {
    let mut ep = Vec::with_capacity(segments.len() * 2);
    for s in segments {
        let mut p1 = s.p1;
        let mut p2 = s.p2;
        // left-lower point precedes
        if (p1.x == p2.x && p1.y > p2.y) || (p1.y == p2.y && p1.x > p2.x) {
            mem::swap(&mut p1, &mut p2);
        }

        if p1.y == p2.y {
            ep.push(EndPoint::from(p1.y, LR(p1.x, p2.x)));
        } else {
            ep.push(EndPoint::from(p1.y, Bottom(p1.x)));
            ep.push(EndPoint::from(p2.y, Top(p2.x)));
        }
    }

    ep.sort();

    let mut bt: BTreeSet<i32> = BTreeSet::new();
    let mut count = 0;
    for p in ep {
        match p.kind {
            Top(x) => {
                bt.remove(&x);
            }
            Bottom(x) => {
                bt.insert(x);
            }
            LR(xl, xr) => {
                count += bt.iter().filter(|&&x| xl <= x && x <= xr).count();
            }
        }
    }
    println!("{}", count);
}

fn main() {
    input! {
        n: usize,
        pairs: [[(i32, i32); 2]; n],
    }

    let ss = pairs
        .iter()
        .map(|v| Segment::from(v[0], v[1]))
        .collect::<Vec<_>>();

    manhattan_intersect(&ss);
}
