#![allow(unused_macros)]
// #![allow(unused_variables)]
#![allow(dead_code)]
// use std::collections::HashSet;
use std::io;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn read_pair<T: FromStr, S: FromStr>() -> (T, S) {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut buf = buf.trim().split_whitespace();

    let t: T = buf.next().unwrap().parse().ok().unwrap();
    let s: S = buf.next().unwrap().parse().ok().unwrap();

    (t, s)
}

// ----------------------------------------------------------------------------------------------------
// main code
// ----------------------------------------------------------------------------------------------------

struct MyHashSet<T> {
    size: usize,
    dict: Vec<T>,
}

trait MyHash {
    fn to_usize(&self) -> usize;
}

impl MyHash for String {
    fn to_usize(&self) -> usize {
        self.chars()
            .collect::<Vec<char>>()
            .iter()
            .map(|&n| match n {
                'A' => 1,
                'T' => 2,
                'C' => 3,
                'G' => 4,
                _ => 0,
            })
            .fold(0, |a, b| 5 * a + b)
    }
}

impl<T> MyHashSet<T>
where
    T: MyHash + Default + Clone + PartialEq,
{
    fn with_capacity(n: usize) -> Self {
        let mut ret = MyHashSet {
            size: n,
            dict: Vec::with_capacity(n),
        };
        ret.dict.resize(n, T::default());
        ret
    }
    fn insert(&mut self, val: T) {
        let key = val.to_usize();
        let h1 = key % self.size;
        let h2 = 1 + (key % (self.size - 1));
        
        for i in 0..self.size {
            let h = (h1 + h2 * i) % self.size;
            if self.dict[h] == val {
                return
            }
            else if self.dict[h] == T::default() {
                self.dict[h] = val.clone();
                return
            }
            // else{
                // conflict: continue loop with different i
            // } 
        }
    }
    fn contains(&self, val: & T) -> bool {
        let key = val.to_usize();
        let h1 = key % self.size;
        let h2 = 1 + (key % (self.size - 1));
        
        for i in 0..self.size {
            let h = (h1 + h2 * i) % self.size;
            if self.dict[h] == *val {
                return true
            }
            else if self.dict[h] == T::default() {
                return false
            }
        }
        false
    }
}

fn solve(n: usize) {
    let size = 1_046_527usize;
    let mut hash_set: MyHashSet<String> = MyHashSet::with_capacity(size);
    for _ in 0..n {
        let (operator, operand) = read_pair::<String, String>();
        let operator = operator.chars().collect::<Vec<char>>();
        match operator[0] {
            'i' => {
                // insert
                hash_set.insert(operand);
            }
            'f' => {
                // find
                if hash_set.contains(&operand) {
                    println!("yes");
                } else {
                    println!("no");
                }
            }
            _ => {
                panic!("met unexpected instruction")
            }
        }
    }
}

fn main() {
    let n = read();
    solve(n);
}
