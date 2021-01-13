#![allow(unused_macros)]
use std::io;
use std::str::FromStr;
use std::string::ToString;
use std::cmp::Ordering;

// ----------------------------------------------------------------------------------------------------
// IO util
// ----------------------------------------------------------------------------------------------------
fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

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
#[derive(Copy, Clone, PartialEq)]
struct Card {
    suit: char,
    number: i32,
}

impl FromStr for Card {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let suit = s.chars().next().unwrap();
        if !['S', 'H', 'C', 'D'].contains(&suit) {
            return Err("met an unknown suit");
        }
        let number: i32 = s[1..].trim().parse().ok().unwrap();
        Ok(Card { suit, number })
    }
}

impl ToString for Card {
    fn to_string(&self) -> String {
        [self.suit.to_string(), self.number.to_string()].join(" ")
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.number.cmp(&other.number))
    }
}

// used for comparison: merge sort is stable
fn merge_sort<T>(ns: &mut Vec<T>, left: usize, right: usize)
where
    T: PartialOrd + Copy + Clone + Max,
{
    if left + 1 >= right {
        return;
    }
    let mid = (left + right) / 2;
    merge_sort(ns, left, mid);
    merge_sort(ns, mid, right);
    merge(ns, left, mid, right);
}

trait Max {
    fn max_value() -> Self;
}

impl Max for Card {
    fn max_value() -> Self {
        Card {
            suit: 'S',
            number: std::i32::MAX,
        }
    }
}

// impl Max for i32 {
//     fn max_value() -> Self {
//         i32::MAX
//     }
// }

fn merge<T>(ns: &mut Vec<T>, left: usize, mid: usize, right: usize)
where
    T: PartialOrd + Copy + Clone + Max,
{
    fn gen_subvector<T>(v: &Vec<T>, start: usize, size: usize) -> Vec<T> where T: Copy + Clone{
        v.iter()
            .skip(start)
            .take(size)
            .map(|&x| x)
            .collect::<Vec<_>>()
    }
    let n1 = mid - left;
    let n2 = right - mid;
    let mut s1 = gen_subvector(ns, left, n1);
    let mut s2 = gen_subvector(ns, mid, n2);
    s1.push(T::max_value());
    s2.push(T::max_value());
    let mut i = 0;
    let mut j = 0;
    for k in left..right {
        if s1[i] <= s2[j] {
            ns[k] = s1[i];
            i += 1;
        } else {
            ns[k] = s2[j];
            j += 1;
        }
    }
}

// returns pivot's index after partitioning
// note that right index is included in range.
fn partition<T>(ns: &mut Vec<T>, left: usize, right: usize) -> usize
where
    T: PartialOrd + Clone + Copy,
{
    let pivot = ns[right];
    let mut i = left;
    for j in left..right {
        if ns[j] <= pivot {
            ns.swap(i, j);
            i += 1;
        }
    }
    ns.swap(i, right);
    i
}

fn quick_sort<T>(ns: &mut Vec<T>, left: usize, right: usize)
where
    T: PartialOrd + Clone + Copy,
{
    if left >= right {
        return;
    }

    let pivot_idx = partition(ns, left, right);
    quick_sort(ns, left, pivot_idx - 1);
    quick_sort(ns, pivot_idx + 1, right);
}

fn is_same<T>(ns1: &Vec<T>, ns2: &Vec<T>) -> bool
where
    T: PartialEq + Copy + Clone,
{
    let matching = ns1.iter().zip(ns2).filter(|(&a, &b)| a == b).count();
    matching == ns1.len() && matching == ns2.len()
}

fn solve<T>(n: usize, ns: &mut Vec<T>)
where
    T: PartialOrd + Copy + Clone + Max + ToString,
{
    let mut stable = ns.clone();
    merge_sort(&mut stable, 0, n);
    quick_sort(ns, 0, ns.len() - 1);

    if is_same(ns, &stable) {
        println!("Stable");
    } else {
        println!("Not stable");
    }

    print_vec(ns, "\n");
}

fn main() {
    let n: usize = read();
    let mut cards: Vec<Card> = (0..n).map(|_| read::<Card>()).collect::<Vec<_>>();
    // print_vec(&cards, "\n");
    solve(n, &mut cards);
}
