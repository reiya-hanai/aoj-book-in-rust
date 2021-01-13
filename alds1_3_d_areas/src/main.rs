use std::collections::VecDeque;
use std::io;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn solve(cs: &Vec<char>) {
    let size = cs.len();
    let mut stack1: VecDeque<usize> = VecDeque::with_capacity(size); // for left's index
    let mut stack2: VecDeque<(usize, usize)> = VecDeque::with_capacity(size); // for (area, left's idx) pair
    for (idx, c) in cs.iter().enumerate() {
        match c {
            '\\' => {
                stack1.push_back(idx);
                // println!("push1: {:?}", stack1);
            }
            '/' => {
                if let Some(left_idx) = stack1.pop_back() {
                    let mut area = idx - left_idx;
                    while let Some((_, l)) = stack2.back() {
                        if left_idx < *l {
                            if let Some((a, _)) = stack2.pop_back() {
                                area += a;
                            }
                        } else {
                            break;
                        }
                    }
                    stack2.push_back((area, left_idx));
                    // println!("push2: {:?}", stack2);
                }
            }
            '_' => {}
            _ => {
                panic!("met an unexpected character")
            }
        }
    }

    // print result

    let (areas, _): (Vec<usize>, Vec<usize>) = stack2.iter().cloned().unzip();
    let total_area = areas.iter().fold(0, |a, b| a + b);
    println!("{}", total_area);
    if stack2.len() > 0 {
        println!(
            "{} {}",
            stack2.len(),
            stack2
                .iter()
                .map(|(a, _)| a.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    } else {
        println!("0")
    }
}

fn main() {
    let s: Vec<char> = read::<String>().chars().collect::<Vec<_>>();
    // println!("{:?}", &s);
    solve(&s);
}
