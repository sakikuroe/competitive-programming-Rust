use proconio::input;
use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    collections::VecDeque,
    usize::MAX,
};

const INF: usize = MAX;

#[derive(Clone)]
struct Edge {
    destination: usize,
    weight: usize,
}

struct Node {
    vertex: usize,
    priority: Reverse<usize>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for Node {}

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
            destination,
            weight,
        }: Edge,
        vertex: usize,
    ) {
        self.edge_list[vertex].push(Edge {
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
            priority: Reverse(0),
        });
        que
    };

    while let Some(Node {
        vertex,
        priority: Reverse(priority),
    }) = que.pop()
    {
        if cost_list[vertex] > priority {
            continue;
        }

        for &Edge {
            destination,
            weight,
        } in &graph.edge_list[vertex]
        {
            if weight != INF && cost_list[destination] > cost_list[vertex] + weight {
                cost_list[destination] = cost_list[vertex] + weight;
                que.push(Node {
                    vertex: destination,
                    priority: Reverse(cost_list[destination]),
                });
            }
        }
    }

    cost_list
}

fn bfs(graph: &Graph, start_list: &Vec<usize>) -> Vec<usize> {
    let mut que: VecDeque<usize> = VecDeque::new();
    let mut distance_sum_list: Vec<usize> = vec![INF; graph.number_of_nodes];

    for &start in start_list {
        que.push_back(start);
        distance_sum_list[start] = 0;
    }
    
    while let Some(current) = que.pop_front() {
        for &Edge {
            destination,
            weight,
        } in &graph.edge_list[current]
        {
            if distance_sum_list[destination] > distance_sum_list[current] + weight {
                distance_sum_list[destination] = distance_sum_list[current] + weight;
                que.push_back(destination);
            }
        }
    }

    distance_sum_list
}

fn calc_edge_list(i: usize, c: usize, r: usize, unweighted_graph: &Graph) -> Vec<Edge> {
    let mut edge_list: Vec<Edge> = vec![];

    let distance_list = bfs(&unweighted_graph, &vec![i]);
    for (j, distance) in distance_list.iter().enumerate() {
        if j != 0 && distance <= &r {
            edge_list.push(Edge {
                destination: j,
                weight: c,
            })
        }
    }
    edge_list
}

fn solve(cr_list: &Vec<(usize, usize)>, ab_list: &Vec<(usize, usize)>) -> usize {
    let n: usize = cr_list.len();

    let graph: Graph = {
        let mut graph = Graph::new(n);
        let unweighted_graph: Graph = {
            let mut graph = Graph::new(n);
            for &(a, b) in ab_list {
                graph.add_edge(
                    Edge {
                        destination: b - 1,
                        weight: 1,
                    },
                    a - 1,
                );
                graph.add_edge(
                    Edge {
                        destination: a - 1,
                        weight: 1,
                    },
                    b - 1,
                );
            }
            graph
        };
        for (i, &(c, r)) in cr_list.iter().enumerate() {
            for edge in calc_edge_list(i, c, r, &unweighted_graph) {
                graph.add_edge(edge, i)
            }
        }
        graph
    };

    dijkstra(&graph, 0)[cr_list.len() - 1]
}

fn output(ans: usize) {
    println!("{}", ans);
}

fn main() {
    input! {
        (n, k): (usize, usize),
        cr_list: [(usize, usize); n],
        ab_list: [(usize, usize); k],
    }

    let ans: usize = solve(&cr_list, &ab_list);

    output(ans);
}
