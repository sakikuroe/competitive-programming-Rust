use proconio::{input, marker::Usize1};
use std::usize::MAX;
const INF: usize = MAX / 3;
const MOD: usize = 998244353;

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
        fn new(n: usize) -> Self {
                let graph = Graph {
                        edges: vec![vec![]; n],
                        n,
                };
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

fn dfs(src: usize, dst: usize, p: usize, cnt_list: &mut Vec<usize>, g: &Graph) -> bool {
        if src == dst {
                return true;
        }

        for &e in &g.edges[src] {
                if e.dst != p {
                        if dfs(e.dst, dst, e.src, cnt_list, g) {
                                cnt_list[e.weight] += 1;
                                return true;
                        }
                }
        }

        false
}

fn main() {
        // Input
        input! {
                n: usize, m: usize, k: isize,
                a_list: [Usize1; m],
                uv_list: [(Usize1, Usize1); n - 1],
        }

        // Initialize
        let mut g = Graph::new(n);
        for (i, (u, v)) in uv_list.into_iter().enumerate() {
                g.add_edge(Edge {
                        src: u,
                        dst: v,
                        weight: i,
                });
                g.add_edge(Edge {
                        src: v,
                        dst: u,
                        weight: i,
                });
        }

        // Solve
        let mut cnt_list = vec![0 as usize; n - 1];
        for i in 0..m - 1 {
                let (src, dst) = (a_list[i], a_list[i + 1]);
                dfs(src, dst, INF, &mut cnt_list, &g);
        }

        let s = cnt_list.iter().sum::<usize>() as isize;

        let ans = if s + k < 0 || (s + k) % 2 == 1 {
                0
        } else {
                let mut dp = vec![0; 100001];
                dp[0] = 1;
                for i in 1..=n - 1 {
                        for j in 0..100000 {
                                let j = 100000 - j - 1;
                                if cnt_list[i - 1] <= j {
                                        dp[j] += dp[j - cnt_list[i - 1]];
                                }
                                dp[j] %= MOD;
                        }
                }

                dp[((s + k) / 2) as usize]
        };

        // Output
        println!("{}", ans);
}
