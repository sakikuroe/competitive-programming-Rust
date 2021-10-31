use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;
use std::usize::MAX;
const INF: usize = MAX / 3;
const MOD: usize = 1_000_000_007;

#[derive(Clone, Copy, Debug)]
struct Edge {
        src: usize,
        dst: usize,
}
#[derive(Debug)]
struct Graph {
        edges: Vec<Vec<Edge>>,
        n: usize,
}
impl Graph {
        fn new(n: usize, sd_dist: &Vec<(usize, usize)>) -> Self {
                let mut graph = Graph {
                        edges: vec![vec![]; n],
                        n,
                };
                for &(s, d) in sd_dist {
                        graph.add_edge(Edge {
                                src: s,
                                dst: d,
                        });
                        graph.add_edge(Edge {
                                src: d,
                                dst: s,
                        });
                }
                graph
        }
        fn add_edge(
                &mut self,
                Edge {
                        src,
                        dst,
                }: Edge,
        ) {
                self.edges[src].push(Edge {
                        src,
                        dst,
                });
        }
}

fn main() {
        // Input
        input! {
                n: usize,
                xy_list: [(Usize1, Usize1); n - 1],
        }

        // Initialize
        let mut stack: Vec<usize> = vec![];
        let g = {
                let g = Graph::new(n, &xy_list);
                let mut res = Graph::new(n, &vec![]);

                let mut que: VecDeque<(usize, usize)> = VecDeque::new();
                que.push_back((0 as usize, INF));

                while let Some((current, p)) = que.pop_front() {
                        stack.push(current);
                        for &Edge {
                                src,
                                dst,
                        } in &g.edges[current]
                        {
                                if dst == p {
                                        continue;
                                }
                                que.push_back((dst, src));
                                res.add_edge(Edge {
                                        src,
                                        dst,
                                });
                        }
                }

                res
        };

        // Solve
        let mut dp = vec![vec![1; 2]; n];
        for i in stack.into_iter().rev().collect::<Vec<usize>>() {
                let mut res0 = 1;
                let mut res1 = 1;

                for &e in &g.edges[i] {
                        res0 *= dp[e.dst][0] % MOD + dp[e.dst][1] % MOD;
                        res0 %= MOD;
                        res1 *= dp[e.dst][0] % MOD;
                        res1 %= MOD;
                }

                dp[i][0] = res0;
                dp[i][1] = res1;
        }
        let ans = dp[0][0] + dp[0][1];

        // Output
        println!("{}", ans % MOD);
}
