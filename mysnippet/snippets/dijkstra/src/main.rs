use cargo_snippet::snippet;

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

use std::usize::MAX;

#[snippet(name = inf, prefix = "use std::usize::MAX;")]
const INF: usize = MAX / 3;

#[snippet(name = "node", prefix = "use std::cmp::{Ordering, Reverse};")]
#[derive(Eq)]
struct Node {
        vertex: usize,
        priory: Reverse<usize>,
}
#[snippet(name = "node")]
impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
                self.priory.cmp(&other.priory)
        }
}
#[snippet(name = "node")]
impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
        }
}
#[snippet(name = "node")]
impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
                self.priory == other.priory
        }
}

#[snippet(name = "edge")]
#[derive(Clone, Copy, Debug)]
struct Edge {
        src: usize,
        dst: usize,
        weight: usize,
}

#[snippet(
        name = "graph",
        include = "inf, edge, node",
        prefix = "use std::collections::BinaryHeap;"
)]
#[derive(Debug)]
struct Graph {
        edges: Vec<Vec<Edge>>,
        n: usize,
}

#[snippet(name = "graph")]
impl Graph {
        fn new(n: usize, sdw_dist: &Vec<(usize, usize, usize)>) -> Self {
                let mut graph = Graph {
                        edges: vec![vec![]; n],
                        n,
                };
                for &(src, dst, weight) in sdw_dist {
                        graph.add_edge(Edge {
                                src,
                                dst,
                                weight,
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
        println!("Hello, world!");
}
