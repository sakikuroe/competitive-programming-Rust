// use proconio::input;
use std::{io, usize::MAX};
const INF: usize = MAX;

use std::collections::VecDeque;

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
                    if board[i][j] == '1' {
                        continue;
                    }
                    for &v in &v_list {
                        let (dy, dx) = (i as isize + v.0, j as isize + v.1);
                        if 0 <= dy
                            && dy < h as isize
                            && 0 <= dx
                            && dx < w as isize
                            && board[dy as usize][dx as usize] != '1'
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
    loop {
        // Input
            // input! {
            //     w: usize, h: usize,
            // }
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let word_list: Vec<&str> = line.split_whitespace().collect();
        let w: usize = word_list[0].parse::<usize>().unwrap();
        let h: usize = word_list[1].parse::<usize>().unwrap();

        if w == 0 && h == 0 {
            break;
        }
        let mut board: Vec<Vec<char>> = vec![];
        for _i in 0..2 * h - 1 {
                // input! {
                //     line: [char; if i % 2 == 0{w - 1}else {w}],
                // }
            line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let line: Vec<char> = {
                let word_list: Vec<&str> = line.split_whitespace().collect();
                word_list
                    .into_iter()
                    .map(|s| s.parse::<char>().unwrap())
                    .collect()
            };
            board.push(line);
        }

        // Initialize
        // 問題を0(道)と1(壁)のマスに置き換える
        // 例えば、Sample Inputの2つ目は以下のようになる
        //
        // 01000100000000000
        // 01111101111101010
        // 01000101000000000
        // 01010101010101111
        // 00000001000001010
        // 01010101111101010
        // 00000000000001000
        //
        let board: Vec<Vec<char>> = {
            let mut res = vec![vec!['0'; 2 * w - 1]; 2 * h - 1];
            for i in 0..h - 1 {
                res[2 * i + 1] = vec!['1'; 2 * w - 1];
            }
            for i in 0..2 * h - 1 {
                if i % 2 == 0 {
                    for j in 0..w - 1 {
                        res[i][j * 2 + 1] = board[i][j];
                    }
                } else {
                    for j in 0..w {
                        res[i][j * 2] = board[i][j];
                    }
                }
            }
            res
        };

        let graph = Graph::new(2 * h - 1, 2 * w - 1, &board);

        // Solve
        // 左上を0としたときの右下までの距離
        let ans = bfs(&graph, &vec![(0, 0)])[2 * h - 2][2 * w - 2];

        if ans == INF {
            println!("0");
        } else {
            println!("{}", ans / 2 + 1);
        }
        // Output
    }
}
