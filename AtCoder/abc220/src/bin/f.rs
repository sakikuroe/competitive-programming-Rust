use proconio::input;
use std::usize::MAX;
const INF: usize = MAX / 3;

#[derive(Clone, Copy, Debug)]
struct Edge {
        src: usize,
        dst: usize,
        weight: usize,
}
#[derive(Clone, Debug)]
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
                                weight: 1,
                        });
                        graph.add_edge(Edge {
                                src: d,
                                dst: s,
                                weight: 1,
                        });
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

fn dfs(src: usize, d: usize, p: usize, sub: &mut Vec<usize>, ans: &mut Vec<usize>, graph: &Graph) {
        ans[0] += d;
        for &edge in &graph.edges[src] {
                if edge.dst == p {
                        continue;
                }
                dfs(edge.dst, d + 1, edge.src, sub, ans, graph);
                sub[edge.src] += sub[edge.dst];
        }
}

fn dfs2(src: usize, p: usize, sub: &mut Vec<usize>, ans: &mut Vec<usize>, graph: &Graph) {
        for &edge in &graph.edges[src] {
                if edge.dst == p {
                        continue;
                }
                ans[edge.dst] = ans[edge.src] + sub.len() - 2 * sub[edge.dst];
                dfs2(edge.dst, edge.src, sub, ans, graph);
        }
}

fn main() {
        // Input
        input! {
                n: usize,
                uv: [(usize, usize); n - 1],
        }

        // Solve
        let uv = uv.into_iter().map(|(a, b)| (a - 1, b - 1)).collect::<Vec<(usize, usize)>>();
        let graph = Graph::new(n, &uv);

        let mut sub = vec![1; n];
        let mut ans = vec![0; n];
        dfs(0, 0, INF, &mut sub, &mut ans, &graph);
        dfs2(0, INF, &mut sub, &mut ans, &graph);

        // Output
        for a in ans {
                println!("{}", a);
        }
}
