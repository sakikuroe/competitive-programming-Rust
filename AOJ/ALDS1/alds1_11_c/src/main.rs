// use proconio::input;
use std::collections::VecDeque;
use std::io;
use std::usize::MAX;
const INF: usize = MAX;

#[derive(Clone, Copy, Debug)]
struct Edge {
        source: usize,
        destination: usize,
        weight: usize,
}

#[derive(Clone, Debug)]
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
        for &start in start_list {
                que.push_back(start);
                dist_list[start] = 0;
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
                                que.push_back(destination);
                        }
                }
        }
        dist_list
}

fn main() {
        // Input
        // input! {
        //         n: usize,
        // }
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let word_list: Vec<&str> = line.split_whitespace().collect();
        let n: usize = word_list[0].parse::<usize>().unwrap();

        // Initialize
        let graph: Graph = {
                let mut graph = Graph::new(n);
                for _ in 0..n {
                        // input! {
                        //         u: usize, k: usize,
                        // }
                        let mut line = String::new();
                        io::stdin().read_line(&mut line).unwrap();
                        let word_list: Vec<&str> = line.split_whitespace().collect();
                        let u: usize = word_list[0].parse::<usize>().unwrap();
                        let k: usize = word_list[1].parse::<usize>().unwrap();

                        for i in 0..k {
                                // input! {
                                //         v: usize,
                                // }
                                let v: usize = word_list[i + 2].parse::<usize>().unwrap();

                                graph.add_edge(Edge {
                                        source: u - 1,
                                        destination: v - 1,
                                        weight: 1,
                                })
                        }
                }

                graph
        };

        // Solve
        let dist_list: Vec<usize> = bfs(&graph, &vec![0 as usize]);

        // Output
        for (i, dist) in dist_list.into_iter().enumerate() {
                println!("{} {}", i + 1, if dist != INF {dist as isize} else {-1} );
        }
}
