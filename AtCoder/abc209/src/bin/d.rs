use proconio::input;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::usize::MAX;
const INF: usize = MAX;

#[derive(Eq)]
struct Node {
        vertex: usize,
        priory: Reverse<usize>,
}
impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
                self.priory.cmp(&other.priory)
        }
}
impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
        }
}
impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
                self.priory == other.priory
        }
}

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

fn dijkstra(graph: &Graph, start: usize) -> Vec<usize> {
        let mut cost_list: Vec<usize> = {
                let mut cost_list = vec![INF; graph.number_of_nodes];
                cost_list[start] = 0;
                cost_list
        };
        let mut que: BinaryHeap<Node> = {
                let mut que = BinaryHeap::new();
                que.push(Node {
                        vertex: start,
                        priory: Reverse(0),
                });
                que
        };
        let mut visited_list: Vec<bool> = vec![false; graph.number_of_nodes];
        while let Some(Node {
                vertex,
                priory: _,
        }) = que.pop()
        {
                if visited_list[vertex] {
                        continue;
                } else {
                        visited_list[vertex] = true;
                }
                for &Edge {
                        source,
                        destination,
                        weight,
                } in &graph.edge_list[vertex]
                {
                        if weight != INF && cost_list[destination] > cost_list[source] + weight {
                                cost_list[destination] = cost_list[source] + weight;
                                que.push(Node {
                                        vertex: destination,
                                        priory: Reverse(cost_list[destination]),
                                });
                        }
                }
        }
        cost_list
}

fn main() {
        // Input
        input! {
            n: usize, q: usize,
            ab_list: [(usize, usize); n - 1],
            cd_list: [(usize, usize); q],
        }

        // Initialize
        // 0-basedに直す
        let ab_list =
                ab_list.into_iter().map(|(a, b)| (a - 1, b - 1)).collect::<Vec<(usize, usize)>>();
        let cd_list =
                cd_list.into_iter().map(|(a, b)| (a - 1, b - 1)).collect::<Vec<(usize, usize)>>();

        // Solve
        let mut graph = Graph::new(n);
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
        let cost_list = dijkstra(&graph, 0);

        let ans_list = {
                let mut res = vec![];
                for (c, d) in cd_list {
                        if (cost_list[c] as isize - cost_list[d] as isize).abs() % 2 == 0 {
                                res.push("Town");
                        } else {
                                res.push("Road");
                        }
                }
                res
        };

        // Output
        for ans in ans_list {
                println!("{}", ans);
        }
}
