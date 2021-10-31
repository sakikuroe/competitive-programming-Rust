use proconio::input;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::usize::MAX;
const INF: usize = MAX / 3;
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
#[derive(Clone, Copy, Debug)]
struct Edge {
        src: usize,
        dst: usize,
        weight: usize,
}
impl Graph {
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
            h: usize, w: usize,
            c_matrix: [[usize; 10]; 10],
            a_matrix: [[isize; w]; h],
        }

        // Initialize
        let graph: Graph = {
                let mut graph: Graph = Graph::new(10, &vec![]);
                for i in 0..10 {
                        for j in 0..10 {
                                graph.add_edge(Edge {
                                        src: j,
                                        dst: i,
                                        weight: c_matrix[i][j],
                                })
                        }
                }
                graph
        };

        // Solve
        let ans: usize = {
                let cost_list = graph.dijkstra(1);

                let mut res: usize = 0;
                for row in a_matrix {
                        for x in row {
                                if x == -1 {
                                        continue;
                                }
                                res += cost_list[x as usize];
                        }
                }

                res
        };

        // Output
        println!("{}", ans);
}
