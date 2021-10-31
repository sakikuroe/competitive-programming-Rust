use proconio::input;
use proconio::marker::Usize1;

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

fn dfs(src: usize, d: usize, ans: &mut usize, visited: &mut Vec<bool>, graph: &Graph) {
        if d == graph.n - 1 {
                *ans += 1;
                return;
        }

        for &edge in &graph.edges[src] {
                if visited[edge.dst] {
                        continue;
                }
                visited[edge.dst] = true;
                dfs(edge.dst, d + 1, ans, visited, graph);
                visited[edge.dst] = false;
        }
}

fn main() {
        // Input
        input! {
                n: usize, m: usize,
                ab_list: [(Usize1, Usize1); m],
        }

        // Initialize
        let g = Graph::new(n, &ab_list);

        // Solve
        let ans = {
                let mut res = 0;
                let mut visited = {
                        let mut res = vec![false; n];
                        res[0] = true;
                        res
                };
                dfs(0, 0, &mut res, &mut visited, &g);
                res
        };

        // Output
        println!("{}", ans);
}
