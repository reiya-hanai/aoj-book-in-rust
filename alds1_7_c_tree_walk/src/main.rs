#![allow(unused_macros)]

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
    parent: Vec<Option<usize>>,
    left: Vec<Option<usize>>,
    right: Vec<Option<usize>>,
}

impl BinaryTree {
    fn new(n: usize) -> Self {
        BinaryTree {
            size: n,
            parent: vec![None; n],
            left: vec![None; n],
            right: vec![None; n],
        }
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

    fn reconstruct(&mut self, preorder: &mut Vec<usize>, inorder: &mut Vec<usize>, parent: Option<usize>) {
        assert_eq!(preorder.len(), inorder.len());
        if preorder.len() == 0 {
            return
        }
        let subroot = preorder[0];
        self.parent[subroot] = parent;
        
        let idx = inorder.iter().position(|x| *x==subroot).unwrap();
        let mut pre_left = preorder.split_off(1);
        let mut pre_right = pre_left.split_off(idx);
        if pre_left.len() > 0 {
            self.left[subroot] = Some(pre_left[0]);
        }
        if pre_right.len() > 0 {
            self.right[subroot] = Some(pre_right[0]);
        }
        
        let mut in_left = inorder;
        let mut in_right = in_left.split_off(idx);
        let mut in_right = in_right.split_off(1);

        self.reconstruct(&mut pre_left, &mut in_left, Some(subroot));
        self.reconstruct(&mut pre_right, &mut in_right, Some(subroot));
    }

    fn walk_postorder(&self, node_id: usize, result: &mut Vec<usize>) {
        if let Some(left) = self.left[node_id] {
            self.walk_postorder(left as usize, result);
        }
        if let Some(right) = self.right[node_id] {
            self.walk_postorder(right as usize, result);
        }
        result.push(node_id + 1);

    }
}

fn solve(n: usize, preorder: &mut Vec<usize>, inorder: &mut Vec<usize>) {
    let mut tree = BinaryTree::new(n);
    tree.reconstruct(preorder, inorder, None);
    let root = tree.get_root();
    let mut result = Vec::new();
    tree.walk_postorder(root, &mut result);
    print_vec(&result, " ");
}

fn main() {
    input! {
        n: usize,
        preorder: [usize1; n],
        inorder: [usize1; n],
    }
    let mut preorder = preorder;
    let mut inorder = inorder;
    solve(n, &mut preorder, &mut inorder);
}
