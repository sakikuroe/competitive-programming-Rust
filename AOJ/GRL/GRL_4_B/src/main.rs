//use proconio::input;
use std::io;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
#[derive(Eq, Debug)]
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
        fn topological_sort(&self) -> Result<Vec<usize>, &str> {
                let mut in_degree = {
                        let mut res = vec![0; self.n];
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
                                let mut que = BinaryHeap::new();
                                for i in 0..self.n {
                                        if in_degree[i] == 0 {
                                                que.push(Node {
                                                        vertex: i,
                                                        priory: Reverse(i),
                                                });
                                        }
                                }
                                
                                que
                        };
                        while let Some(Node {
                                vertex,
                                priory: _priory,
                        }) = que.pop()
                        {
                                for &e in &self.edges[vertex] {
                                        in_degree[e.dst] -= 1;
                                        if in_degree[e.dst] == 0 {
                                                que.push(Node {
                                                        vertex: e.dst,
                                                        priory: Reverse(e.dst),
                                                })
                                        }
                                }
                                sorted.push(vertex);
                        }

                        if sorted.len() == self.n {
                                Ok(sorted)
                        } else {
                                Err("-1")
                        }
                };

                res
        }
}
fn main() {
        // input! {
        //         v_size: usize, e_size: usize,
        //         st_list: [(usize, usize); e_size],
        // }
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        let v_size: usize = iter.next().unwrap().parse::<usize>().unwrap();
        let e_size: usize = iter.next().unwrap().parse::<usize>().unwrap();
        
        let mut st_list: Vec<(usize, usize)> = Vec::new();
        for _ in 0..e_size {
            line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let word_list: Vec<usize> = {
                let word_list: Vec<&str> = line.split_whitespace().collect();
                word_list
                    .into_iter()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect()
            };
            st_list.push((word_list[0], word_list[1]));
        }

        let  g = Graph::new(v_size, &st_list);
        let ans = g.topological_sort().unwrap();
        
        for a in ans {
                println!("{}", a);
        }
}
