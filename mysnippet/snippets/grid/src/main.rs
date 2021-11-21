use cargo_snippet::snippet;

use std::collections::BinaryHeap;
use std::cmp::{Ordering, Reverse};

use std::usize::MAX;

#[snippet(name = inf, prefix = "use std::usize::MAX;")]
const INF: usize = MAX/3;


#[snippet(name = "node2", prefix = "use std::cmp::{Ordering, Reverse};")]
#[derive(Eq)]
struct Node2 {
        vertex: (usize, usize),
        priory: Reverse<usize>,
}
#[snippet(name = "node2")]
impl Ord for Node2 {
        fn cmp(&self, other: &Self) -> Ordering {
                self.priory.cmp(&other.priory)
        }
}
#[snippet(name = "node2")]
impl PartialOrd for Node2 {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
        }
}
#[snippet(name = "node2")]
impl PartialEq for Node2 {
        fn eq(&self, other: &Self) -> bool {
                self.priory == other.priory
        }
}

#[snippet(name = "edge2")]
#[derive(Clone, Copy, Debug)]
struct Edge2 {
        src: (usize, usize),
        dst: (usize, usize),
        weight: usize,
}

#[snippet(name = "graph2", include = "inf, edge2, node2",prefix = "use std::collections::BinaryHeap;")]
#[snippet(name = "maze")]
#[derive(Debug)]
struct Graph2 {
        edges: Vec<Vec<Vec<Edge>>>,
        h: usize,
        w: usize,
}


#[snippet(name = "graph2")]
#[snippet(name = "maze")]
impl Graph2 {
        fn new_maze(h: usize, w: usize, maze: &Vec<Vec<char>>) -> Self {
                let mut graph = Graph {
                        edges: vec![vec![vec![]; w]; h],
                        h,
                        w,
                };
                let v_list = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
                for i in 0..h {
                        for j in 0..w {
                                if maze[i][j] == '#' {
                                        continue;
                                }
                                for &v in &v_list {
                                        let (ny, nx) = (i as isize + v.0, j as isize + v.1);
                                        if 0 <= ny
                                                && ny < h as isize
                                                && 0 <= nx
                                                && nx < w as isize
                                                && maze[ny as usize][nx as usize] != '#'
                                        {
                                                graph.add_edge(Edge {
                                                        src: (i, j),
                                                        dst: (ny as usize, nx as usize),
                                                        weight: 1,
                                                })
                                        }
                                }
                        }
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
                self.edges[src.0][src.1].push(Edge {
                        src,
                        dst,
                        weight,
                });
        }

        fn dijkstra(self, start: (usize, usize)) -> Vec<Vec<usize>> {
                let mut dist = {
                        let mut dist = vec![vec![INF; self.w]; self.h];
                        dist[start.0][start.1] = 0;
                        dist
                };
                let mut que: BinaryHeap<Node> = {
                        let mut que = BinaryHeap::new();
                        que.push(Node {
                                vertex: (start.0, start.1),
                                priory: Reverse(0),
                        });
                        que
                };
                let mut visited = vec![vec![false; self.w]; self.h];
                while let Some(Node {
                        vertex,
                        priory: _,
                }) = que.pop()
                {
                        if visited[vertex.0][vertex.1] {
                                continue;
                        }
                        for &Edge {
                                src,
                                dst,
                                weight,
                        } in &self.edges[vertex.0][vertex.1]
                        {
                                if dist[dst.0][dst.1] > dist[src.0][src.1] + weight {
                                        dist[dst.0][dst.1] = dist[src.0][src.1] + weight;
                                        que.push(Node {
                                                vertex: dst,
                                                priory: Reverse(dist[dst.0][dst.1]),
                                        });
                                }
                        }
                        visited[vertex.0][vertex.1] = true;
                }
                dist
        }
}



fn main() {
        let g = Graph::new_maze(2, 2, &vec![vec!['2'; 2]; 2]);
}
