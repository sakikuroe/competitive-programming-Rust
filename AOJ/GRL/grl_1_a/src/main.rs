//use proconio::input;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io;
use std::usize::MAX;
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

#[derive(Eq, PartialEq, PartialOrd)]
struct Node {
        vertex: usize,
        priory: usize,
}

impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
                self.priory.cmp(&other.priory).reverse()
        }
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
        // input! {
        //     v_size: usize,
        //     e_size: usize,
        //     r: usize,
        //     std_list: [(usize, usize, usize); e_size],
        // }
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let word_list: Vec<&str> = line.split_whitespace().collect();
        let v_size: usize = word_list[0].parse::<usize>().unwrap();
        let e_size: usize = word_list[1].parse::<usize>().unwrap();
        let r: usize = word_list[2].parse::<usize>().unwrap();

        let mut std_list: Vec<(usize, usize, usize)> = Vec::new();
        for _ in 0..e_size {
                line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                let word_list: Vec<usize> = {
                        let word_list: Vec<&str> = line.split_whitespace().collect();
                        word_list.into_iter().map(|s| s.parse::<usize>().unwrap()).collect()
                };
                std_list.push((word_list[0], word_list[1], word_list[2]));
        }

        // Initialize
        let graph: Graph = {
                let mut graph = Graph::new(v_size);
                for (s, t, d) in std_list {
                        graph.add_edge(Edge {
                                src: s,
                                dst: t,
                                weight: d,
                        });
                }
                graph
        };
        let start_position: usize = r;

        // Solve
        let cost_list: Vec<usize> = graph.dijkstra(start_position);

        // Output
        for cost in cost_list {
                if cost == INF {
                        println!("INF");
                } else {
                        println! {"{}", cost};
                }
        }
}
