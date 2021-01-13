use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::str::FromStr;

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
    Find(T),
    Delete(T),
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
                "find" => Command::Find(operand),
                "delete" => Command::Delete(operand),
                _ => panic! {"error while parsing commands"},
            }
        };
        match command {
            Some("print") => Ok(Command::Print),
            Some(name) if ["insert", "find", "delete"].contains(&name) => {
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

type Link<T> = Rc<RefCell<Node<T>>>;

#[derive(Clone)]
struct Node<T>
where
    T: Default + Clone + PartialOrd,
{
    val: T,
    left: Option<Link<T>>,
    right: Option<Link<T>>,
    parent: Option<Link<T>>,
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
            parent: None,
        }
    }

    fn insert(&mut self, key: T) {
        if key < self.val {
            match &self.left {
                Some(left) => {
                    let mut left = left.borrow_mut();
                    (*left).insert(key);
                }
                None => {
                    self.left = Some(Rc::new(RefCell::new(Node::new(key))));
                }
            }
        } else {
            match &self.right {
                Some(right) => {
                    let mut right = right.borrow_mut();
                    (*right).insert(key);
                }
                None => {
                    self.right = Some(Rc::new(RefCell::new(Node::new(key))));
                }
            }
        }
    }

    fn find(&self, key: T) -> Option<Link<T>> {
        if key == self.val {
            Some(Rc::new(RefCell::new(self.clone())))
        } else if key < self.val {
            if let Some(left) = &self.left {
                let left = left.borrow();
                (*left).find(key)
            } else {
                None
            }
        } else {
            if let Some(right) = &self.right {
                let right = right.borrow();
                (*right).find(key)
            } else {
                None
            }
        }
    }
}

struct BST<T>
where
    T: Default + Clone + PartialOrd + fmt::Display,
{
    root: Option<Link<T>>,
}

impl<T> BST<T>
where
    T: Default + Clone + PartialOrd + fmt::Display,
{
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, key: T) {
        if let Some(link) = &self.root {
            let mut node = link.borrow_mut();
            (*node).insert(key);
        } else {
            self.root = Some(Rc::new(RefCell::new(Node::new(key))));
        }
    }

    fn find(&self, key: T) -> Option<Link<T>> {
        if let Some(link) = &self.root {
            let node = link.borrow();
            (*node).find(key)
        } else {
            None
        }
    }

    fn delete(&mut self, _key: T) {
        // if let Some(root) = &self.root {
        //     let root = root.borrow();
        //     if let Some(node) = (*root).find(key) {
        //         // let node_to_delete = self.find_for_delete(node);
        //     }
        // }
    }

    // fn find_for_delete(&self, node: Link<T>) -> Option<Link<T>> {
        
    // }

    fn print_inorder(node: &Option<Link<T>>) {
        if let Some(ref v) = node {
            let v = v.borrow();
            Self::print_inorder(&v.left);
            print!(" {}", v.val);
            Self::print_inorder(&v.right);
        }
    }

    fn print_preorder(node: &Option<Link<T>>) {
        if let Some(ref v) = node {
            let v = v.borrow();
            print!(" {}", v.val);
            Self::print_preorder(&v.left);
            Self::print_preorder(&v.right);
        }
    }

    fn print(&self) {
        Self::print_inorder(&self.root);
        println!("");
        Self::print_preorder(&self.root);
        println!("");
    }
}

use Command::*;

fn solve(n: usize) {
    let mut tree = BST::new();
    (0..n).for_each(|_| {
        let op: Command<i32> = read();
        match op {
            Insert(key) => tree.insert(key),
            Find(key) => {
                if tree.find(key).is_some() {
                    println!("yes")
                } else {
                    println!("no")
                }
            }
            Delete(key) => {
                tree.delete(key);
            }
            Print => tree.print(),
        }
    })
}

fn main() {
    let n: usize = read();
    solve(n);
}
