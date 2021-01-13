#![allow(unused_macros)]
// #![allow(unused_variables)]
#![allow(dead_code)]
use std::collections::HashSet;
use std::str::FromStr;
use std::io;

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

fn solve(n: usize) {
    let size = 1_046_527usize;
    let mut hash_set: HashSet<String> = HashSet::with_capacity(size);
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
                }
                else {
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
