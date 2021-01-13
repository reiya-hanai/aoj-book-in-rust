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

struct Solver {
    count: usize,
    array: Vec<i32>,
}
impl Solver {
    fn new(array: Vec<i32>) -> Self {
        Self { count: 0, array }
    }

    pub fn run(&mut self) {
        self.merge_sort(0, self.array.len());
        print_vec(&self.array, " ");
        println!("{}", self.count);
    }

    // after this method call, self.array[left..right] is sorted
    fn merge_sort(&mut self, left: usize, right: usize) {
        if left + 1 >= right {
            return;
        }
        let mid = (left + right) / 2;
        self.merge_sort(left, mid);
        self.merge_sort(mid, right);
        self.merge(left, mid, right);
    }

    fn merge(&mut self, left: usize, mid: usize, right: usize) {
        fn gen_subvector(v: &Vec<i32>, start: usize, size: usize) -> Vec<i32> {
            v.iter()
                .skip(start)
                .take(size)
                .map(|&x| x)
                .collect::<Vec<_>>()
        }
        let n1 = mid - left;
        let n2 = right - mid;
        let mut s1 = gen_subvector(&self.array, left, n1);
        let mut s2 = gen_subvector(&self.array, mid, n2);
        s1.push(std::i32::MAX);
        s2.push(std::i32::MAX);
        let mut i = 0;
        let mut j = 0;
        for k in left..right {
            self.count += 1;
            if s1[i] <= s2[j] {
                self.array[k] = s1[i];
                i += 1;
            } else {
                self.array[k] = s2[j];
                j += 1;
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        s: [i32; n],
    }
    let mut solver = Solver::new(s);
    solver.run();
}
