use proconio::input;
use std::cmp;

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
        fn new(n: usize, sd_list: &Vec<(usize, usize)>) -> Self {
                let mut graph = Graph {
                        edges: vec![vec![]; n],
                        n,
                };
                for &(s, d) in sd_list {
                        graph.add_edge(Edge {
                                src: s,
                                dst: d,
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
                n: usize, m: usize,
                ab_list: [(usize, usize); m],
        }

        // Initialize
        // 0-basedに直す
        let ab_list =
                ab_list.into_iter().map(|(a, b)| (a - 1, b - 1)).collect::<Vec<(usize, usize)>>();
        let g = Graph::new(n, &ab_list);

        let n1 = (n + 1) / 2;
        let n2 = n / 2;

        let mut ok = vec![true; 1 << n1];
        for v in 0..n1 {
                for &Edge {
                        src,
                        dst,
                } in &g.edges[v]
                {
                        if dst < n1 {
                                ok[(1 << src) | (1 << dst)] = false;
                        }
                }
        }
        for bit in 0..(1 << n1) {
                if ok[bit] == false {
                        for w in 0..n1 {
                                ok[bit | (1 << w)] = false;
                        }
                }
        }

        let mut set = vec![(1 << n2) - 1; 1 << n1];
        for v in 0..n1 {
                set[1 << v] = (1 << n2) - 1;
                for &Edge {
                        src,
                        dst,
                } in &g.edges[v]
                {
                        if src < n1 && n1 <= dst {
                                set[1 << src] &= !(1 << (dst - n1));
                        }
                }
        }
        for bit in 0..(1 << n1) {
                for w in 0..n1 {
                        set[bit | (1 << w)] = set[bit] & set[1 << w];
                }
        }

        let mut ok2 = vec![true; 1 << n2];
        for v in 0..n2 {
                for &Edge {
                        src,
                        dst,
                } in &g.edges[v + n1]
                {
                        if n1 <= src {
                                ok2[(1 << (src - n1)) | (1 << (dst - n1))] = false;
                        }
                }
        }
        for bit in 0..(1 << n2) {
                if !ok2[bit] {
                        for w in 0..n2 {
                                ok2[bit | (1 << w)] = false;
                        }
                }
        }

        let mut dp = vec![0; 1 << n2];
        for bit in 0..(1 << n2) {
                if ok2[bit] {
                        dp[bit] = (bit as u64).count_ones();
                }
        }
        for bit in 0..(1 << n2) {
                for w in 0..n2 {
                        dp[bit | (1 << w)] = cmp::max(dp[bit | (1 << w)], dp[bit]);
                }
        }

        // Solve
        let mut ans = 0;
        for bit in 0..(1 << n1) {
                if !ok[bit] {
                        continue;
                }
                ans = cmp::max(ans, (bit as u64).count_ones() + dp[set[bit]]);
        }

        // Output
        println!("{}", ans);
}
