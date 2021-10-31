use proconio::{input, marker::Chars};
use std::cmp::{Ordering, Reverse};
#[derive(Eq)]
struct Node {
        vertex: (usize, usize),
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

use std::usize::MAX;
const INF: usize = MAX;
use std::collections::BinaryHeap;

fn dijkstra(graph: &Graph, start: (usize, usize)) -> usize {
        let mut res = 0;
        let mut cost_list = {
                let mut cost_list = vec![vec![INF; graph.w]; graph.h];
                cost_list[start.0][start.1] = 0;
                cost_list
        };
        let mut que: BinaryHeap<Node> = {
                let mut que = BinaryHeap::new();
                que.push(Node {
                        vertex: (start.0, start.1),
                        priory: Reverse(0),
                });
                que
        };
        let mut visited_list = vec![vec![false; graph.w]; graph.h];
        while let Some(Node {
                vertex,
                priory: _,
        }) = que.pop()
        {
                if visited_list[vertex.0][vertex.1] {
                        continue;
                } else {
                        visited_list[vertex.0][vertex.1] = true;
                        res = std::cmp::max(res, cost_list[vertex.0][vertex.1]);
                }
                for &Edge {
                        source,
                        destination,
                        weight,
                } in &graph.edge_list[vertex.0][vertex.1]
                {
                        if weight != INF
                                && cost_list[destination.0][destination.1]
                                        > cost_list[source.0][source.1] + weight
                        {
                                cost_list[destination.0][destination.1] =
                                        cost_list[source.0][source.1] + weight;
                                que.push(Node {
                                        vertex: destination,
                                        priory: Reverse(cost_list[destination.0][destination.1]),
                                });
                        }
                }
        }
        res
}

#[derive(Clone, Copy, Debug)]
struct Edge {
        source: (usize, usize),
        destination: (usize, usize),
        weight: usize,
}
#[derive(Debug)]
struct Graph {
        edge_list: Vec<Vec<Vec<Edge>>>,
        h: usize,
        w: usize,
}
impl Graph {
        fn new(h: usize, w: usize, board: &Vec<Vec<char>>) -> Self {
                let graph = {
                        let mut graph = Graph {
                                edge_list: vec![vec![vec![]; w]; h],
                                h,
                                w,
                        };
                        let v_list: Vec<(isize, isize)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
                        for i in 0..h {
                                for j in 0..w {
                                        if board[i][j] == '#' {
                                                continue;
                                        }
                                        for &v in &v_list {
                                                let (dy, dx) = (i as isize + v.0, j as isize + v.1);
                                                if 0 <= dy
                                                        && dy < h as isize
                                                        && 0 <= dx
                                                        && dx < w as isize
                                                        && board[dy as usize][dx as usize]!='#'
                                                {
                                                        graph.add_edge(Edge {
                                                                source: (i, j),
                                                                destination: (
                                                                        dy as usize,
                                                                        dx as usize,
                                                                ),
                                                                weight: 1,
                                                        })
                                                }
                                        }
                                }
                        }
                        graph
                };
                graph
        }
        fn add_edge(
                &mut self,
                Edge {
                        source: (y, x),
                        destination,
                        weight,
                }: Edge,
        ) {
                self.edge_list[y][x].push(Edge {
                        source: (y, x),
                        destination,
                        weight,
                });
        }
}

fn main() {
        // Input
        input! {
            h: usize, w: usize,
            s_mat: [Chars; h],
        }

        // Initialize
        let graph = Graph::new(h, w, &s_mat);

        // Solve
        let ans_list = {
                let mut res = vec![];
                for i in 0..h {
                        for j in 0..w {
                                if s_mat[i][j] == '#' {
                                        continue;
                                }
                                let ans = dijkstra(&graph, (i, j));
                                res.push(ans);
                        }
                }
                res
        };
        let ans = ans_list.into_iter().max().unwrap();

        // Output
        println!("{}", ans);
}
