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

struct Heap<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Heap<T>
where
    T: Clone + Copy + ToString + Ord,
{
    fn new(n: usize, keys: Vec<T>) -> Self {
        Self {
            size: n,
            data: keys,
        }
    }

    fn print(&self) {
        print!(" ");
        print_vec(&self.data, " ");
    }

    fn left(i: usize) -> usize {
        2 * i + 1
    }

    fn right(i: usize) -> usize {
        2 * i + 2
    }

    // Make SubTree (whose root is i) a max-heap
    fn max_heapify(&mut self, i: usize) {
        let mut largest = i;
        let l = Heap::<T>::left(i);
        let r = Heap::<T>::right(i);
        if l < self.size && self.data[i] < self.data[l] {
            largest = l;
        }
        if r < self.size && self.data[largest] < self.data[r] {
            largest = r;
        }
        if largest != i {
            self.data.swap(largest, i);
            self.max_heapify(largest);
        }
    }

    fn build_max_heap(&mut self) {
        if self.size != 0 {
            for i in (0..(self.size - 1)).rev() {
                self.max_heapify(i);
            }
        }
    }
}

fn solve(n: usize, keys: Vec<i32>) {
    let mut heap = Heap::new(n, keys);
    heap.build_max_heap();
    heap.print();
}

fn main() {
    input! {
        n: usize,
        ks: [i32; n],
    }
    solve(n, ks);
}
