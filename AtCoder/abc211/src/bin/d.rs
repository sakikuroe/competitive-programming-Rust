use proconio::input;
use std::collections::VecDeque;
use std::usize::MAX;
const INF: usize = MAX;
const MOD: usize = 1000000007;
#[derive(Clone, Copy, Debug)]
struct Edge {
        source: usize,
        destination: usize,
        weight: usize,
}
struct Graph {
        edge_list: Vec<Vec<Edge>>,
        number_of_nodes: usize,
}
impl Graph {
        fn new(number_of_nodes: usize) -> Self {
                Graph {
                        edge_list: vec![vec![]; number_of_nodes],
                        number_of_nodes,
                }
        }
        fn add_edge(
                &mut self,
                Edge {
                        source,
                        destination,
                        weight,
                }: Edge,
        ) {
                self.edge_list[source].push(Edge {
                        source,
                        destination,
                        weight,
                });
        }
}

fn bfs(graph: &Graph, start_list: &Vec<usize>) -> Vec<usize> {
        let mut que: VecDeque<usize> = VecDeque::new();
        let mut dist_list: Vec<usize> = vec![INF; graph.number_of_nodes];
        let mut cnt_list: Vec<usize> = vec![0; graph.number_of_nodes];

        for &start in start_list {
                que.push_back(start);
                dist_list[start] = 0;
                cnt_list[start] = 1;
        }
        while let Some(current) = que.pop_front() {
                for &Edge {
                        source,
                        destination,
                        weight,
                } in &graph.edge_list[current]
                {
                        if dist_list[destination] > dist_list[source] + weight {
                                dist_list[destination] = dist_list[source] + weight;
                                cnt_list[destination] += cnt_list[source] % MOD;
                                que.push_back(destination);
                        } else if dist_list[destination] == dist_list[source] + weight {
                                cnt_list[destination] += cnt_list[source] % MOD;
                        }
                }
        }
        cnt_list
}

fn main() {
        // Input
        input! {
                n: usize, m: usize,
                ab_list: [(usize, usize); m],
        }

        // Initialize
        let ab_list =
                ab_list.into_iter().map(|(a, b)| (a - 1, b - 1)).collect::<Vec<(usize, usize)>>();

        // Solve
        let mut graph: Graph = Graph::new(n);
        for (a, b) in ab_list {
                graph.add_edge(Edge {
                        source: a,
                        destination: b,
                        weight: 1,
                });
                graph.add_edge(Edge {
                        source: b,
                        destination: a,
                        weight: 1,
                });
        }

        let ans = bfs(&graph, &vec![0])[n - 1];

        // Output
        println!("{}", ans % MOD);
}
