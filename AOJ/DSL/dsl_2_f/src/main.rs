// use proconio::input;
use std::cmp;
use std::io;
const INF: usize = (1 << 31) - 1;
#[derive(Clone, Debug)]
struct Node {
        value: usize,
        lazy: usize,
}
impl Node {
        fn new() -> Node {
                Node {
                        value: INF,
                        lazy: INF,
                }
        }
}
#[derive(Debug)]
struct LazySegmentTree {
        size: usize,
        nodes: Vec<Node>,
}
impl LazySegmentTree {
        fn new(n: usize) -> Self {
                let size = 2u32.pow(((n - 1) as f64).log2() as u32 + 1) as usize;
                LazySegmentTree {
                        size,
                        nodes: vec![Node::new(); 2 * size - 1],
                }
        }
        fn eval(&mut self, idx: usize) {
                if self.nodes[idx].lazy != INF {
                        self.nodes[idx].value = self.nodes[idx].lazy;

                        if idx < self.size - 1 {
                                self.nodes[2 * idx + 1].lazy = self.nodes[idx].lazy;
                                self.nodes[2 * idx + 2].lazy = self.nodes[idx].lazy;
                        }
                        self.nodes[idx].lazy = INF;
                }
        }
        fn get_value(&mut self, idx: usize) -> usize {
                self.eval(idx);
                self.nodes[idx].value
        }
        fn update_sub(&mut self, a: usize, b: usize, idx: usize, x: usize, l: usize, r: usize) {
                self.eval(idx);
                if a <= l && r <= b {
                        self.nodes[idx].lazy = x;
                } else if a < r && l < b {
                        self.update_sub(a, b, 2 * idx + 1, x, l, (l + r) / 2);
                        self.update_sub(a, b, 2 * idx + 2, x, (l + r) / 2, r);
                        self.nodes[idx].value =
                                cmp::min(self.get_value(2 * idx + 1), self.get_value(2 * idx + 2));
                }
        }
        fn update(&mut self, l: usize, r: usize, x: usize) {
                self.update_sub(l, r, 0, x, 0, self.size);
        }
        fn find_sub(&mut self, a: usize, b: usize, idx: usize, l: usize, r: usize) -> usize {
                self.eval(idx);
                if a <= l && r <= b {
                        return self.nodes[idx].value;
                } else if a < r && l < b {
                        return cmp::min(
                                self.find_sub(a, b, 2 * idx + 1, l, (l + r) / 2),
                                self.find_sub(a, b, 2 * idx + 2, (l + r) / 2, r),
                        );
                } else {
                        return INF;
                }
        }
        fn find(&mut self, l: usize, r: usize) -> usize {
                self.find_sub(l, r, 0, 0, self.size)
        }
}

fn main() {
        // Input
        // input! {
        //         n: usize, q: usize,
        // }
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse::<usize>().unwrap();
        let q: usize = iter.next().unwrap().parse::<usize>().unwrap();

        // Initialize
        let mut seg_tree = LazySegmentTree::new(n);

        // Solve
        let ans_list = {
                let mut res: Vec<usize> = vec![];
                for _ in 0..q {
                        // input! {
                        //         f: usize,
                        // }
                        let mut line = String::new();
                        io::stdin().read_line(&mut line).unwrap();
                        let mut iter = line.split_whitespace();
                        let f: usize = iter.next().unwrap().parse::<usize>().unwrap();

                        if f == 0 {
                                // input! {
                                //         s:usize, t: usize, x: usize,
                                // }
                                let s: usize = iter.next().unwrap().parse::<usize>().unwrap();
                                let t: usize = iter.next().unwrap().parse::<usize>().unwrap();
                                let x: usize = iter.next().unwrap().parse::<usize>().unwrap();
                                seg_tree.update(s, t + 1, x);
                        } else {
                                // input! {
                                //         s:usize, t: usize,
                                // }
                                let s: usize = iter.next().unwrap().parse::<usize>().unwrap();
                                let t: usize = iter.next().unwrap().parse::<usize>().unwrap();
                                res.push(seg_tree.find(s, t + 1));
                        }
                }

                res
        };

        // Output
        for ans in ans_list {
                println!("{}", ans);
        }
}
