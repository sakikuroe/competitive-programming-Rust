use proconio::input;
use proconio::marker::Usize1;

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::usize::MAX;
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
const INF: usize = MAX / 3;
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
        fn new(n: usize, sdw_list: &Vec<(usize, usize, usize)>) -> Self {
                let mut graph = Graph {
                        edges: vec![vec![]; n],
                        n,
                };
                for &(s, d, weight) in sdw_list {
                        graph.add_edge(Edge {
                                src: s,
                                dst: d,
                                weight,
                        });
                        graph.add_edge(Edge {
                                src: d,
                                dst: s,
                                weight,
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
        fn dijkstra(self, start: usize) -> Vec<usize> {
                let mut dist = {
                        let mut dist = vec![INF; self.n];
                        dist[start] = 0;
                        dist
                };
                let mut que: BinaryHeap<Node> = {
                        let mut que = BinaryHeap::new();
                        que.push(Node {
                                vertex: start,
                                priory: Reverse(0),
                        });
                        que
                };
                let mut visited = vec![false; self.n];
                while let Some(Node {
                        vertex,
                        priory: _,
                }) = que.pop()
                {
                        if visited[vertex] {
                                continue;
                        }
                        for &Edge {
                                src,
                                dst,
                                weight,
                        } in &self.edges[vertex]
                        {
                                if dist[dst] > dist[src] + weight {
                                        dist[dst] = dist[src] + weight;
                                        que.push(Node {
                                                vertex: dst,
                                                priory: Reverse(dist[dst]),
                                        });
                                }
                        }
                        visited[vertex] = true;
                }
                dist
        }
}

fn main() {
        // Input
        input! {
                n: usize,
                uvw_list: [(Usize1, Usize1, usize); n - 1],
        }

        // Initialize
        let g = Graph::new(n, &uvw_list);

        // Solve
        let ans = {
                let cost_list = g.dijkstra(0);

                let mut res = vec![];
                for cost in cost_list {
                        if cost % 2 == 0 {
                                res.push(0);
                        } else {
                                res.push(1);
                        }
                }

                res
        };

        // Output
        for a in ans {
                println!("{}", a);
        }
}
