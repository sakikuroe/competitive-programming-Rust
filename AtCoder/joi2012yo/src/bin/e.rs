use proconio::input;
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
    fn new(h: usize, w: usize, board: &Vec<Vec<usize>>) -> Self {
        let graph = {
            let mut graph = Graph {
                edge_list: vec![vec![vec![]; w]; h],
                h,
                w,
            };
            for i in 0..h {
                for j in 0..w {
                    if board[i][j] == 1 {
                        continue;
                    }
                    let v_list: Vec<(isize, isize)> = if i % 2 == 0 {
                        vec![(0, 1), (-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0)]
                    } else {
                        vec![(0, 1), (-1, 1), (-1, 0), (0, -1), (1, 0), (1, 1)]
                    };
                    for &v in &v_list {
                        let (dy, dx) = (i as isize + v.0, j as isize + v.1);
                        if 0 <= dy
                            && dy < h as isize
                            && 0 <= dx
                            && dx < w as isize

                        {if board[dy as usize][dx as usize] != 1{
                            graph.add_edge(Edge {
                                source: (i, j),
                                destination: (dy as usize, dx as usize),
                                weight: 1,
                            })    
                        } else {
                            graph.add_edge(Edge {
                                source: (i, j),
                                destination: (dy as usize, dx as usize),
                                weight: INF,
                            })       
                        }

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

fn bfs(graph: &Graph, start_list: &Vec<(usize, usize)>) -> usize {
    let mut res : usize = 0;
    let mut que: VecDeque<(usize, usize)> = VecDeque::new();
    let mut dist_list: Vec<Vec<usize>> = vec![vec![INF; graph.w]; graph.h];
    for &(sy, sx) in start_list {
        que.push_back((sy, sx));
        dist_list[sy][sx] = 0;
    }
    while let Some((current_y, current_x)) = que.pop_front() {
        for &Edge {
            source: (_sy, _sx),
            destination: (dy, dx),
            weight,
        } in &graph.edge_list[current_y][current_x]
        {
            if dist_list[dy][dx] == INF {

                if weight == INF {
                    res += 1;
                } else {
                    dist_list[dy][dx] = 0;
                    que.push_back((dy, dx));
                }
            }
        }
    }
    res
}


fn main() {
    // Input
    input! {
        w: usize, h: usize,
        board:[[usize;w];h],
    }


    
    // Initialize
    // boradを[[usize; w + 2]; h + 2]に拡張し、内側にもとのboardを入れる
    // 一番外側の周は0にする
    // 
    //              0 0 0 0
    //   1 0        0 1 0 0
    //   0 1   ->   0 0 1 0
    //   1 1        0 1 1 0
    //              0 0 0 0
    
    let board:Vec<Vec<usize>> = {
        let mut new_board:Vec<Vec<usize>> = vec![vec![0 as usize; w+2];h+2];
        for i in 0..h {
            for j in 0..w {
                new_board[i +1][j+1] = board[i][j];
            }
        }
        new_board
    };

    // 0 -> 0の辺の重さを1、 0 -> 1の辺の重さをINFに設定する
    let graph: Graph = Graph::new(h + 2, w + 2, &board);

    // Solve
    // (0, 0) から初めて、INFの重みの辺を発見したらカウントする
    let ans = bfs(&graph, &vec![(0,0)]);

    // Output
    println!("{}", ans);
}
