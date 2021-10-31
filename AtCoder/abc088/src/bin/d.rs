use proconio::{input, marker::Chars};
use std::collections::VecDeque;
use std::usize::MAX;
const INF: usize = MAX;

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
                            && board[dy as usize][dx as usize] != '#'
                        {
                            graph.add_edge(Edge {
                                source: (i, j),
                                destination: (dy as usize, dx as usize),
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

fn bfs(graph: &Graph, start_list: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut que: VecDeque<(usize, usize)> = VecDeque::new();
    let mut dist_list: Vec<Vec<usize>> = vec![vec![INF; graph.w]; graph.h];
    for &(sy, sx) in start_list {
        que.push_back((sy, sx));
        dist_list[sy][sx] = 0;
    }
    while let Some((current_y, current_x)) = que.pop_front() {
        for &Edge {
            source: (sy, sx),
            destination: (dy, dx),
            weight,
        } in &graph.edge_list[current_y][current_x]
        {
            if dist_list[dy][dx] > dist_list[sy][sx] + weight {
                dist_list[dy][dx] = dist_list[sy][sx] + weight;
                que.push_back((dy, dx));
            }
        }
    }
    dist_list
}

fn main() {
    // Input
    input! {
        h: usize, w:usize,
        s_mat:[Chars;h],
    }

    // Initialize
    // black := s_matに最初からある黒いマス('#')の数
    let black: usize = {
        let mut cnt: usize = 0;
        for i in 0..h {
            for j in 0..w {
                if s_mat[i][j] == '#' {
                    cnt += 1;
                }
            }
        }
        cnt
    };

    let graph: Graph = Graph::new(h, w, &s_mat);

    // Solve
    let min_path = bfs(&graph, &vec![(0, 0)])[h - 1][w - 1];

    let ans: isize = if min_path == INF {
        -1
    } else {
        // すべてのマス目の数から、最初から黒かったマスと必ず通らなければならないマスの数を引く
        h as isize * w as isize - black as isize - (min_path + 1) as isize
    };

    // Output
    println!("{}", ans);
}
