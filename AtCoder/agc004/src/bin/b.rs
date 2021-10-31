use proconio::input;
use std::cmp;
use std::usize::MAX;
const INF: usize = MAX / 3;

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
        input! {
                n: usize,
                x: usize,
                a_list: [usize; n],
        }

        // Initialize
        let segment_tree = {
                let mut res = SegmentTree::new(n);
                for i in 0..n {
                        res.update(i, a_list[i]);
                }

                res
        };

        // Solve
        let ans = {
                let mut v: Vec<usize> = vec![];

                for i in 0..n {
                        let mut t = 0;
                        t += i * x;
                        for j in 0..n {
                                if i <= j {
                                        t += segment_tree.find(j - i, j + 1)
                                } else {
                                        t += cmp::min(
                                                segment_tree.find(0, j + 1),
                                                segment_tree.find(n - (i - j), n + 1),
                                        );
                                }
                        }

                        v.push(t);
                }

                v.into_iter().min().unwrap()
        };

        // Output
        println!("{}", ans);
}
