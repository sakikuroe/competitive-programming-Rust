use proconio::input;
use proconio::marker::Usize1;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize::MAX;
#[derive(Eq, PartialOrd, PartialEq)]
struct Node {
        vertex: usize,
        priory: usize,
}
impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
                self.priory.cmp(&other.priory).reverse()
        }
}
const INF: usize = MAX / 3;
#[derive(Debug)]
struct Graph {
        edges: Vec<Vec<Edge>>,
        size: usize,
}
impl Graph {
        fn new(size: usize) -> Self {
                Graph {
                        edges: vec![vec![]; size],
                        size,
                }
        }
        fn add_edge(&mut self, e: Edge) {
                self.edges[e.src].push(Edge {
                        src: e.src,
                        dst: e.dst,
                        weight: e.weight,
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
        fn dijkstra(&self, start: usize) -> Vec<usize> {
                let mut dist = vec![INF; self.size];
                dist[start] = 0;
                let mut que = BinaryHeap::new();
                que.push(Node {
                        vertex: start,
                        priory: 0,
                });
                while let Some(node) = que.pop() {
                        if node.priory < dist[node.vertex] {
                                continue;
                        }
                        for &e in &self.edges[node.vertex] {
                                if dist[e.dst] > dist[e.src] + e.weight {
                                        dist[e.dst] = dist[e.src] + e.weight;
                                        que.push(Node {
                                                vertex: e.dst,
                                                priory: dist[e.dst],
                                        });
                                }
                        }
                }
                dist
        }
}
fn main() {
        // Input
        input! {
            n: usize, k:usize,
        }

        // Solve
        let mut g = Graph::new(n);
        for _ in 0..k {
                input! {
                    f: usize,
                }
                if f == 0 {
                        input! {
                            a: Usize1,
                            b: Usize1,
                        }
                        let dist = g.dijkstra(a)[b];
                        if dist != INF {
                                println!("{}", dist);
                        } else {
                                println!("-1");
                        }
                } else if f == 1 {
                        input! {
                            c: Usize1,
                            d: Usize1,
                            e: usize,
                        }
                        g.add_edge(Edge {
                                src: c,
                                dst: d,
                                weight: e,
                        });
                        g.add_edge(Edge {
                                src: d,
                                dst: c,
                                weight: e,
                        });
                }
        }
}
