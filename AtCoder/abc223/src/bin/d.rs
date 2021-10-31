use proconio::{input, marker::Usize1};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
#[derive(Eq, Debug)]
struct Vertex {
        priory: usize,
}
impl Ord for Vertex {
        fn cmp(&self, other: &Self) -> Ordering {
                self.priory.cmp(&other.priory).reverse()
        }
}
impl PartialOrd for Vertex {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
        }
}
impl PartialEq for Vertex {
        fn eq(&self, other: &Self) -> bool {
                self == other
        }
}
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
impl Graph {
        fn topological_sort(&self) -> Result<Vec<usize>, &str> {
                let mut indegree = {
                        let mut res = vec![0; self.size];
                        for edge_list in &self.edges {
                                for &e in edge_list {
                                        res[e.dst] += 1;
                                }
                        }
                        res
                };
                let res = {
                        let mut sorted: Vec<usize> = vec![];
                        let mut que = {
                                let mut res = BinaryHeap::new();
                                for i in 0..self.size {
                                        if indegree[i] == 0 {
                                                res.push(Vertex {
                                                        priory: i,
                                                });
                                        }
                                }
                                res
                        };
                        while let Some(Vertex {
                                priory,
                        }) = que.pop()
                        {
                                for &e in &self.edges[priory] {
                                        indegree[e.dst] -= 1;
                                        if indegree[e.dst] == 0 {
                                                que.push(Vertex {
                                                        priory: e.dst,
                                                })
                                        }
                                }
                                sorted.push(priory);
                        }

                        if sorted.len() == self.size {
                                Ok(sorted)
                        } else {
                                Err("-1")
                        }
                };
                res
        }
}
fn main() {
        // Input
        input! {
                n: usize, m: usize,
                ab_list: [(Usize1, Usize1); m],
        }

        // Solve
        let mut g = Graph::new(n);
        for (a, b) in ab_list {
                g.add_edge(Edge {
                        src: a,
                        dst: b,
                        weight: 1,
                });
        }
        let ans = g.topological_sort();

        // Output
        match ans {
                Ok(v) => {
                        let v = v.into_iter().map(|x| x + 1).collect::<Vec<usize>>();
                        for i in 0..n {
                                print!(
                                        "{}{}",
                                        v[i],
                                        if i != n - 1 {
                                                " "
                                        } else {
                                                "\n"
                                        }
                                );
                        }
                }
                Err(s) => println!("{}", s),
        }
}
