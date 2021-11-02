// use proconio::input;
use std::cmp;
use std::io;
const INF: usize = (1 << 31) - 1;

#[derive(Clone, Copy, Debug)]
struct Edge {
        src: usize,
        dst: usize,
        weight: usize,
}
#[derive(Debug)]
struct SegmentTree {
        size: usize,
        nodes: Vec<usize>,
}
impl SegmentTree {
        fn new(n: usize) -> Self {
                let size = (2 as u32).pow(((n - 1) as f64).log2() as u32 + 1) as usize;
                SegmentTree {
                        size,
                        nodes: vec![INF; 2 * size - 1],
                }
        }
        fn update(&mut self, mut idx: usize, x: usize) {
                idx += self.size - 1;
                self.nodes[idx] = x;

                while idx != 0 {
                        idx = (idx - 1) / 2;
                        self.nodes[idx] =
                                cmp::min(self.nodes[2 * idx + 1], self.nodes[2 * idx + 2]);
                }
        }
        fn find_sub(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> usize {
                if r <= a || b <= l {
                        return INF;
                }
                if a <= l && r <= b {
                        return self.nodes[k];
                } else {
                        return cmp::min(
                                self.find_sub(a, b, k * 2 + 1, l, (l + r) / 2),
                                self.find_sub(a, b, k * 2 + 2, (l + r) / 2, r),
                        );
                }
        }
        fn find(&self, l: usize, r: usize) -> usize {
                self.find_sub(l, r, 0, 0, self.size)
        }
}

fn main() {
        // Input
        // input! {
        //         n: usize, q: usize,
        //         cxy_list: [(usize, usize, usize); q],
        // }
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse::<usize>().unwrap();
        let q: usize = iter.next().unwrap().parse::<usize>().unwrap();

        let mut cxy_list: Vec<(usize, usize, usize)> = Vec::new();
        for _ in 0..q {
                line = String::new();
                io::stdin().read_line(&mut line).unwrap();
                let word_list: Vec<usize> = {
                        let word_list: Vec<&str> = line.split_whitespace().collect();
                        word_list.into_iter().map(|s| s.parse::<usize>().unwrap()).collect()
                };
                cxy_list.push((word_list[0], word_list[1], word_list[2]));
        }

        // Initialize
        let mut segment_tree = SegmentTree::new(n);

        // Solve
        let ans = {
                let mut res = vec![];
                for (com, x, y) in cxy_list {
                        if com == 0 {
                                segment_tree.update(x, y);
                        } else {
                                res.push(segment_tree.find(x, y + 1));
                        }
                }

                res
        };

        // Output
        for a in ans {
                println!("{}", a);
        }
}
