#![allow(unused_macros)]
use std::io;
use std::str::FromStr;
use std::string::ToString;

// ----------------------------------------------------------------------------------------------------
// IO util
// ----------------------------------------------------------------------------------------------------
fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

// read whitespace-separated vector descrived in horizontal direction
fn read_vec_h<T: FromStr>() -> Vec<T> {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();
    let mut ts = Vec::new();

    for t_str in buf.trim().split_whitespace() {
        let t: T = t_str.parse().ok().unwrap();
        ts.push(t)
    }

    ts
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
struct RootedTree {
    size: usize,
    parent: Vec<Option<usize>>,
    left_child: Vec<Option<usize>>,
    right_sibling: Vec<Option<usize>>,
}

impl RootedTree {
    fn from(triple: &Vec<(usize, usize, Vec<usize>)>) -> Self {
        let n = triple.len();
        let mut ret = RootedTree {
            size: n,
            parent: vec![None; n],
            left_child: vec![None; n],
            right_sibling: vec![None; n],
        };

        for (id, degree, childs) in triple {
            if *degree > 0 {
                ret.left_child[*id] = Some(childs[0]);
                let mut right = None;
                for child_id in childs.iter().rev() {
                    ret.parent[*child_id] = Some(*id);
                    ret.right_sibling[*child_id] = right;
                    right = Some(*child_id);
                }
            }
        }

        ret
    }

    // fn get_root(&self) -> usize {
    //     let roots = self.parent.iter().enumerate().filter(|(_, v)| v.is_none()).collect::<Vec<_>>();
    //     debug_assert!(roots.len() == 1);
    //     roots[0].0
    // }

    fn get_depth(&self, id: usize) -> usize {
        let mut depth = 0;
        let mut cur_id = id;
        while let Some(p) = self.parent[cur_id] {
            cur_id = p;
            depth += 1;
        }
        depth
    }

    fn print_node(&self, id: usize) {
        let node_type = match self.parent[id] {
            None => "root",
            Some(_) => match self.left_child[id] {
                None => "leaf",
                Some(_) => "internal node",
            },
        };
        let parent = match self.parent[id] {
            None => -1,
            Some(p) => p as i32,
        };
        let mut childs = Vec::new();
        let mut cur_child = self.left_child[id];
        while let Some(id) = cur_child {
            childs.push(id);
            cur_child = self.right_sibling[id];
        }
        println!(
            "node {}: parent = {}, depth = {}, {}, {:?}",
            id,
            parent,
            self.get_depth(id),
            node_type,
            childs
        )
    }

    fn print(&self) {
        (0..self.size).for_each(|i| self.print_node(i));
    }
}

fn solve(_n: usize, triple: &Vec<(usize, usize, Vec<usize>)>) {
    let tree = RootedTree::from(triple);
    // println!{"{:?}", tree};

    tree.print();
}

fn main() {
    let n: usize = read();
    let mut rows = Vec::with_capacity(n);
    for _ in 0..n {
        let tmp = read_vec_h();
        let id = tmp[0];
        let degree = tmp[1];
        let mut childs = Vec::new();
        for j in 0..degree {
            childs.push(tmp[2 + j])
        }
        rows.push((id, degree, childs));
    }

    solve(n, &rows);
}
