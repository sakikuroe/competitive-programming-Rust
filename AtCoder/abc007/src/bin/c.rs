use proconio::{input, marker::Chars};
use std::{collections::VecDeque, usize::MAX};

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
    fn new(h: usize, w: usize) -> Self {
        Graph {
            edge_list: vec![vec![vec![]; w]; h],
            h,
            w,
        }
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

// 到達するのにかかる最小のコストを返す
// 到達できない場合はINFを返す
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

fn solve(
    r: usize,
    c: usize,
    sy: usize,
    sx: usize,
    gy: usize,
    gx: usize,
    c_mat: &Vec<Vec<char>>,
) -> usize {
    let graph = {
        let mut graph = Graph::new(r, c);
        let v_list: Vec<(isize, isize)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        for i in 0..r {
            for j in 0..c {
                if c_mat[i][j] == '#' {
                    continue;
                }
                for &v in &v_list {
                    let (dy, dx) = (i as isize + v.0, j as isize + v.1);
                    if 0 <= dy
                        && dy <= r as isize
                        && 0 <= dx
                        && dx <= c as isize
                        && c_mat[dy as usize][dx as usize] != '#'
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
    // start_list := 探索を開始する座標のリスト
    // 今回の場合は1点(sy, sx)
    let start_list: Vec<(usize, usize)> = vec![(sy, sx)];
    // dist_list[i][j] = (i, j)に到達するのにかかる最小のコスト
    let dist_list: Vec<Vec<usize>> = bfs(&graph, &start_list);
    dist_list[gy][gx]
}

fn output(ans: usize) {
    println!("{}", ans);
}

fn main() {
    // Input
    input! {
        r: usize, c: usize,
        sy: usize, sx: usize,
        gy: usize, gx: usize,
        c_mat: [Chars; r],
    }

    // Initialize
    // 0-basedに変更する
    let (sy, sx) = (sy - 1, sx - 1);
    let (gy, gx) = (gy - 1, gx - 1);

    // Solve
    let ans: usize = solve(r, c, sy, sx, gy, gx, &c_mat);

    // Output
    output(ans);
}
