use proconio::{input, marker::Usize1};
use std::collections::{HashSet, VecDeque};
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
                        graph.add_edge(Edge {
                                src: d,
                                dst: s,
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
struct State {
        board: [usize; 9],
        prev: usize,
        deep: usize,
}
fn bfs(graph: &Graph, board: &[usize; 9]) -> isize {
        let mut que: VecDeque<State> = VecDeque::new();
        que.push_back(State {
                board: *board,
                prev: INF,
                deep: 0,
        });

        let mut checked: HashSet<[usize; 9]> = HashSet::new();
        checked.insert(*board);

        while let Some(s) = que.pop_front() {
                if s.board == [0, 1, 2, 3, 4, 5, 6, 7, 8] {
                        return s.deep as isize;
                } else {
                        let space = {
                                let mut res = INF;
                                for i in 0..9 {
                                        if s.board[i] == 8 {
                                                res = i;
                                                break;
                                        }
                                }
                                res
                        };
                        for e in &graph.edges[space] {
                                let new_board = {
                                        let mut res = s.board;
                                        res[e.src] = res[e.dst];
                                        res[e.dst] = 8;
                                        res
                                };
                                if !checked.contains(&new_board) {
                                        checked.insert(new_board);
                                        que.push_back(State {
                                                board: new_board,
                                                prev: e.src,
                                                deep: s.deep + 1,
                                        });
                                }
                        }
                }
        }

        return -1;
}
fn main() {
        // Input
        input! {
                m: usize,
                uv_list: [(Usize1, Usize1); m],
                p_list: [Usize1;8],
        }

        // Initialize
        let mut board = [8; 9];
        for i in 0..8 {
                board[p_list[i]] = i;
        }

        // Solve
        let uvw_list =
                uv_list.into_iter().map(|x| (x.0, x.1, 1)).collect::<Vec<(usize, usize, usize)>>();
        let g = Graph::new(9, &uvw_list);
        let ans = bfs(&g, &mut board);

        // Output
        println!("{}", ans);
}
