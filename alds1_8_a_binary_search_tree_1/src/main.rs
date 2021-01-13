use std::str::FromStr;
use std::fmt;

// ----------------------------------------------------------------------------------------------------
// IO util
// ----------------------------------------------------------------------------------------------------
fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
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

#[derive(Debug)]
enum Command<T> {
    Insert(T),
    Print,
}

impl<T> FromStr for Command<T>
where
    T: FromStr,
{
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.trim().split_whitespace();
        let command = s.next();
        fn get_command<T>(s: &str, operand: T) -> Command<T> {
            match s {
                "insert" => Command::Insert(operand),
                _ => panic! {"error while parsing commands"},
            }
        };
        match command {
            Some("print") => Ok(Command::Print),
            Some(name) if name == "insert" => {
                let operand = s.next();
                match operand {
                    Some(n) => Ok(get_command(name, n.parse().ok().unwrap())),
                    _ => Err("error while parsing commands"),
                }
            }
            _ => Err("error while parsing commands"),
        }
    }
}

type Link<T> = Box<Node<T>>;

struct Node<T>
where
    T: Default + Clone + PartialOrd,
{
    val: T,
    left: Option<Link<T>>,
    right: Option<Link<T>>,
}

impl<T> Node<T>
where
    T: Default + Clone + PartialOrd,
{
    fn new(v: T) -> Self {
        Self {
            val: v,
            left: None,
            right: None,
        }
    }
}

struct BinarySearchTree<T>
where
    T: Default + Clone + PartialOrd + fmt::Display,
{
    root: Option<Link<T>>,
}

impl<T> BinarySearchTree<T>
where
    T: Default + Clone + PartialOrd + fmt::Display,
{
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, key: T) {
        let z = Box::new(Node::new(key));
        let mut current = &mut self.root;

        while let Some(node) = current {
            if z.val < node.val {
                current = &mut node.left;
            } else {
                current = &mut node.right;
            }
        }
        *current = Some(z);
    }

    fn print_inorder(node: &Option<Link<T>>) {
        if let Some(ref v) = node {
            Self::print_inorder(&v.left);
            print!(" {}", v.val);
            Self::print_inorder(&v.right);
        }
    }

    fn print_preorder(node: &Option<Link<T>>) {
        if let Some(ref v) = node {
            print!(" {}", v.val);
            Self::print_preorder(&v.left);
            Self::print_preorder(&v.right);
        }
    }

    fn print(&mut self) {
        Self::print_inorder(&self.root);
        println!("");
        Self::print_preorder(&self.root);
        println!("");
    }
}

use Command::*;

fn solve(n: usize) {
    let mut tree = BinarySearchTree::new();
    (0..n).for_each(|_| {
        let op: Command<i32> = read();
        match op {
            Insert(key) => tree.insert(key),
            Print => tree.print(),
        }
    })
}

fn main() {
    let n: usize = read();
    solve(n);
}
