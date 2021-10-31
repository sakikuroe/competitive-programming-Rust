use proconio::input;
use std::cmp;

struct UnionFind {
        parent: Vec<usize>,
        rank: Vec<usize>,
        size: Vec<usize>,
        #[allow(unused)]
        n: usize,
}
impl UnionFind {
        fn new(number_of_nodes: usize) -> Self {
                let parent = (0..number_of_nodes).collect::<Vec<usize>>();
                let rank = vec![0; number_of_nodes];
                let size = vec![1; number_of_nodes];
                UnionFind {
                        parent,
                        rank,
                        size,
                        n: number_of_nodes,
                }
        }
        fn is_root(&mut self, x: usize) -> bool {
                self.parent[x] == x
        }
        fn root(&mut self, x: usize) -> usize {
                if self.is_root(x) {
                        x
                } else {
                        self.parent[x] = self.root(self.parent[x]);
                        self.parent[x]
                }
        }
        fn size(&mut self, x: usize) -> usize {
                let a = self.root(x);
                if self.is_root(a) {
                        self.size[a]
                } else {
                        self.size(a)
                }
        }
        fn unite(&mut self, x: usize, y: usize) {
                let x = self.root(x);
                let y = self.root(y);
                if x != y {
                        if self.rank[x] < self.rank[y] {
                                self.size[y] += self.size(x);
                                self.parent[x] = y;
                        } else {
                                self.size[x] += self.size(y);
                                self.parent[y] = x;
                                if self.rank[x] == self.rank[y] {
                                        self.rank[x] += 1;
                                }
                        }
                }
        }
        fn same(&mut self, x: usize, y: usize) -> bool {
                self.root(x) == self.root(y)
        }
}
#[derive(Clone, Copy, Debug)]
struct Edge {
        src: usize,
        dst: usize,
        weight: isize,
}
fn kruskal(edge_list: &mut Vec<Edge>, v_size: usize) -> isize {
        let mut path_weight_sum: isize = 0;
        let mut unionfind = UnionFind::new(v_size);
        for Edge {
                src,
                dst,
                weight,
        } in edge_list
        {
                if !unionfind.same(*src, *dst) {
                        unionfind.unite(*src, *dst);
                        path_weight_sum += *weight;
                        *weight = 0;
                } else {
                        if *weight < 0 {
                                *weight = 0;
                        }
                }
        }
        path_weight_sum
}

fn main() {
        // Input
        input! {
                n: usize, m: usize,
                abc_list: [(usize, usize, isize); m],
        }

        // Initialize
        let abc_list =
                abc_list.iter()
                        .map(|&(a, b, c)| (a - 1, b - 1, c))
                        .collect::<Vec<(usize, usize, isize)>>();

        // Solve
        let mut edges = {
                let mut res = vec![];
                for &(a, b, c) in &abc_list {
                        res.push(Edge {
                                src: a,
                                dst: b,
                                weight: c,
                        });
                }
                res
        };
        edges.sort_by_key(|&a| a.weight);

        kruskal(&mut edges, n);

        // Output
        println!("{}", edges.iter().map(|x| cmp::max(x.weight, 0)).fold(0, |x, y| x + y));
}
