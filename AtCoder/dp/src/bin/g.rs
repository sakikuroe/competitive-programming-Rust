use memoise::memoise;
use proconio::{input, marker::Usize1};
use std::cmp;

#[derive(Clone, Copy, Debug)]
struct Edge {
        src: usize,
        dst: usize,
        weight: usize,
}
#[derive(Debug)]
struct Graph {
        edges: Vec<Vec<Edge>>,
        n: usize,
}
impl Graph {
        fn new(n: usize, sd_list: &Vec<(usize, usize)>) -> Self {
                let mut graph = Graph {
                        edges: vec![vec![]; n],
                        n,
                };
                for &(src, dst) in sd_list {
                        graph.add_edge(Edge {
                                src,
                                dst,
                                weight: 1,
                        })
                }
                graph
        }
        fn add_edge(
                &mut self,
                Edge {
                        src,
                        dst,
                        weight,
                }: Edge,
        ) {
                self.edges[src].push(Edge {
                        src,
                        dst,
                        weight,
                });
        }
}

#[memoise (0 <= n <= 100000)]
// dp[n] := n(0-based)から始まる経路の最大長
fn dp(n: usize, g: &Graph) -> usize {
        if g.edges[n].len() == 0 {
                return 0;
        } else {
                let mut res = 0;
                for &Edge {
                        src: _,
                        dst,
                        weight: _,
                } in &g.edges[n]
                {
                        res = cmp::max(res, dp(dst, g) + 1);
                }
                return res;
        }
}

fn main() {
        // Input
        input! {
                n: usize, m: usize,
                xy_list: [(Usize1, Usize1); m],
        }

        // Initialize
        let g = Graph::new(n, &xy_list);

        // Solve
        let ans = {
                let mut res = 0;
                for i in 0..n {
                        res = cmp::max(res, dp(i, &g));
                }
                res
        };

        // Output
        println!("{}", ans);
}
