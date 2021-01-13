#![allow(unused_macros)]
use std::cmp;

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
// IO util
// ----------------------------------------------------------------------------------------------------
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
struct BinaryTree {
    size: usize,
    parent: Vec<Option<i32>>,
    left: Vec<Option<i32>>,
    right: Vec<Option<i32>>,
    depth: Vec<usize>,
    height: Vec<usize>,
}

impl BinaryTree {
    fn from(triple: &Vec<(usize, i32, i32)>) -> Self {
        let n = triple.len();
        let mut ret = BinaryTree {
            size: n,
            parent: vec![None; n],
            left: vec![None; n],
            right: vec![None; n],
            depth: vec![0; n],
            height: vec![0; n],
        };

        // set parent, left, right
        for (id, left, right) in triple {
            if *left >= 0 {
                ret.left[*id] = Some(*left);
                ret.parent[*left as usize] = Some(*id as i32);
            }
            if *right >= 0 {
                ret.right[*id] = Some(*right);
                ret.parent[*right as usize] = Some(*id as i32);
            }
        }

        ret
    }

    fn set_depth_recur(&mut self, id: usize, d: usize) {
        self.depth[id] = d;
        if let Some(l) = self.left[id] {
            self.set_depth_recur(l as usize, d + 1)
        }
        if let Some(r) = self.right[id as usize] {
            self.set_depth_recur(r as usize, d + 1)
        }
    }

    fn set_depth(&mut self) {
        let root = self.get_root();
        self.set_depth_recur(root, 0);
    }

    fn set_height_recur(&mut self, id: usize) -> usize {
        let ret = match (self.left[id], self.right[id]) {
            (Some(l), Some(r)) => {
                let hl = self.set_height_recur(l as usize) + 1;
                let hr = self.set_height_recur(r as usize) + 1;
                cmp::max(hl, hr)
            }
            (Some(l), None) => self.set_height_recur(l as usize) + 1,
            (None, Some(r)) => self.set_height_recur(r as usize) + 1,
            (None, None) => 0,
        };
        self.height[id] = ret;
        ret
    }

    fn set_height(&mut self) {
        let root = self.get_root();
        self.set_height_recur(root);
    }

    fn print_node(&self, id: usize) {
        let node_type = match (self.parent[id], self.left[id], self.right[id]) {
            (None, _, _) => "root",
            (Some(_), None, None) => "leaf",
            (_, _, _) => "internal node",
        };
        let parent = match self.parent[id] {
            None => -1,
            Some(p) => p as i32,
        };
        let sibling = match self.parent[id] {
            None => -1,
            Some(p) => match (self.left[p as usize], self.right[p as usize]) {
                (None, Some(_)) => -1,
                (Some(_), None) => -1,
                (Some(l), Some(r)) if l as usize == id => r,
                (Some(l), Some(r)) if r as usize == id => l,
                (_, _) => panic!("unexpected error while searching sibling")
            }
        };

        let degree = match (self.left[id], self.right[id]) {
            (None, None) => 0,
            (Some(_), None) => 1,
            (None, Some(_)) => 1,
            (Some(_), Some(_)) => 2,
        };

        println!(
            "node {}: parent = {}, sibling = {}, degree = {}, depth = {}, height = {}, {}",
            id,
            parent,
            sibling,
            degree,
            self.depth[id],
            self.height[id],
            node_type,
        )
    }

    fn get_root(&self) -> usize {
        let roots = self
            .parent
            .iter()
            .enumerate()
            .filter(|(_, v)| v.is_none())
            .collect::<Vec<_>>();
        debug_assert!(roots.len() == 1);
        roots[0].0
    }

    fn print(&self) {
        (0..self.size).for_each(|i| self.print_node(i));
    }
}

fn solve(_n: usize, triple: &Vec<(usize, i32, i32)>) {
    let mut tree = BinaryTree::from(triple);
    tree.set_depth();
    tree.set_height();
    // println! {"{:?}", tree};

    tree.print();
}

fn main() {
    input! {
        n: usize,
        rows: [(usize, i32, i32); n],
    }
    // println!("{:?}", rows);
    solve(n, &rows);
}
