#![allow(dead_code)]
use std::str::FromStr;
use std::string::ToString;

// ----------------------------------------------------------------------------------------------------
// IO util
// ----------------------------------------------------------------------------------------------------
#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_vec_v<T: FromStr>(size: usize) -> Vec<T> {
    (0..size).map(|_| read()).collect::<Vec<_>>()
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

#[derive(Debug)]
enum Command<T> {
    Insert(T),
    Extract,
    End,
}

impl<T> FromStr for Command<T>
where
    T: FromStr,
{
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.trim().split_whitespace();
        let command = s.next();
        match command {
            Some("extract") => Ok(Command::Extract),
            Some("end") => Ok(Command::End),
            Some("insert") => {
                let operand = s.next();
                match operand {
                    Some(n) => Ok(Command::Insert(n.parse().ok().unwrap())),
                    _ => Err("error while parsing operand"),
                }
            }
            _ => Err("error while parsing commands"),
        }
    }
}

impl<T> ToString for Command<T>
where
    T: ToString,
{
    fn to_string(&self) -> String {
        match self {
            Command::Insert(n) => String::from("insert ") + &n.to_string(),
            Command::Extract => String::from("extract"),
            Command::End => String::from("end"),
        }
    }
}

fn left(i: usize) -> usize {
    2 * i + 1
}

fn right(i: usize) -> usize {
    2 * i + 2
}

fn parent(i: usize) -> Option<usize> {
    if i > 0 {
        Some((i - 1) / 2)
    } else {
        None
    }
}

struct Heap<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Heap<T>
where
    T: Clone + Copy + ToString + Ord + Default,
{
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    // Make SubTree (whose root is i) a max-heap
    fn max_heapify(&mut self, i: usize) {
        let mut largest = i;
        let l = left(i);
        let r = right(i);
        if l < self.size && self.data[i] < self.data[l] {
            largest = l;
        }
        if r < self.size && self.data[largest] < self.data[r] {
            largest = r;
        }
        if largest != i {
            self.data.swap(largest, i);
            self.max_heapify(largest);
        }
    }

    fn insert(&mut self, v: T) {
        self.size += 1;
        if self.size > self.data.len() {
            self.data.resize(self.size, T::default());
        }
        self.data[self.size - 1] = v;
        let mut cur = self.size - 1;
        loop {
            if let Some(p) = parent(cur) {
                if self.data[cur] > self.data[p] {
                    self.data.swap(cur, p);
                    cur = p;
                } else {
                    // ordering done
                    break;
                }
            } else {
                // root
                break;
            }
        }
    }

    fn delete_max(&mut self) -> T {
        let ret = self.data[0];
        self.data.swap(0, self.size - 1);
        self.size -= 1;
        self.max_heapify(0);
        ret
    }
}

fn solve() {
    let mut heap = Heap::new();
    loop {
        let c: Command<u32> = read();
        use Command::*;
        match c {
            Insert(v) => heap.insert(v),
            Extract => {
                let v = heap.delete_max();
                println!("{}", v);
            }
            End => break,
        }
    }
}

fn main() {
    solve();
}
