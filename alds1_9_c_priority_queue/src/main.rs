#![allow(dead_code)]
use std::collections::BinaryHeap;
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


fn solve() {
    let mut heap = BinaryHeap::new();
    loop {
        let c: Command<u32> = read();
        use Command::*;
        match c {
            Insert(v) => heap.push(v),
            Extract => {
                let v = heap.pop();
                println!("{}", v.unwrap());
            }
            End => break,
        }
    }
}

fn main() {
    solve();
}
