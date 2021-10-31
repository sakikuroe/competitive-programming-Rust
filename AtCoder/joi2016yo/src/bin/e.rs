use proconio::input;
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, VecDeque},
    usize::MAX,
};

const INF: usize = MAX;

#[derive(Clone, Copy)]
struct Edge {
    source: usize,
    destination: usize,
    weight: usize,
}

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

    while let Some(Node { vertex, priory: _ }) = que.pop() {
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

fn bfs(graph: &Graph, start_list: &Vec<usize>) -> Vec<usize> {
    let mut que: VecDeque<usize> = VecDeque::new();
    let mut safety_level_list: Vec<usize> = vec![INF; graph.number_of_nodes];

    for &start in start_list {
        que.push_back(start - 1);
        safety_level_list[start - 1] = 0;
    }

    while let Some(current) = que.pop_front() {
        for &Edge {
            source,
            destination,
            weight,
        } in &graph.edge_list[current]
        {
            if safety_level_list[destination] > safety_level_list[source] + weight {
                safety_level_list[destination] = safety_level_list[source] + weight;
                que.push_back(destination);
            }
        }
    }

    safety_level_list
}

fn calc_hotel_charge(safety_level: usize, s: usize, p: usize, q: usize) -> usize {
    if safety_level == INF {
        0
    } else if safety_level > s {
        p
    } else if safety_level > 0 {
        q
    } else {
        INF
    }
}

fn solve(
    n: usize,
    s: usize,
    p: usize,
    q: usize,
    c_list: &Vec<usize>,
    ab_list: &Vec<(usize, usize)>,
) -> usize {
    let safety_level_list: Vec<usize> = {
        let graph = {
            let mut graph: Graph = Graph::new(n);
            for &(a, b) in ab_list {
                graph.add_edge(Edge {
                    source: a - 1,
                    destination: b - 1,
                    weight: 1,
                });
                graph.add_edge(Edge {
                    source: b - 1,
                    destination: a - 1,
                    weight: 1,
                });
            }

            graph
        };

        let mut safety_level_list: Vec<usize> = bfs(&graph, &c_list);
        safety_level_list[0] = INF;
        safety_level_list[n - 1] = INF;

        safety_level_list
    };

    let graph = {
        let mut graph = Graph::new(n);
        for &(a, b) in ab_list {
            graph.add_edge(Edge {
                source: a - 1,
                destination: b - 1,
                weight: calc_hotel_charge(safety_level_list[b - 1], s, p, q),
            });
            graph.add_edge(Edge {
                source: b - 1,
                destination: a - 1,
                weight: calc_hotel_charge(safety_level_list[a - 1], s, p, q),
            });
        }
        graph
    };

    dijkstra(&graph, 0)[n - 1]
}

fn output(ans: usize) {
    println!("{}", ans);
}

fn main() {
    input! {
        (n, m, k, s): (usize, usize, usize, usize),
        (p, q): (usize, usize),
        c_list: [usize; k],
        ab_list: [(usize, usize); m],
    }

    let ans: usize = solve(n, s, p, q, &c_list, &ab_list);

    output(ans);
}
