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

fn solve(graph: &Graph, cheese_list: &Vec<(usize, usize)>) -> usize {
    let mut ans: usize = 0;
    for i in 0..cheese_list.len() - 1 {
        let (s, g) = (cheese_list[i], cheese_list[i + 1]);
        ans += bfs(graph, &vec![s])[g.0][g.1];
    }
    ans
}

fn main() {
    // Input
    input! {
        h: usize, w:usize,n:usize,
        board: [Chars; h],
    }

    // Initialize
    let cheese_list = {
        // cheese_list[0] = Sの座標
        // cheese_list[i] = i(0 <= i <= n)の座標
        let mut cheese_list: Vec<(usize, usize)> = vec![(0, 0); n + 1];
        for i in 0..h {
            for j in 0..w {
                if '1' <= board[i][j] && board[i][j] <= '9' {
                    cheese_list[board[i][j] as usize - '0' as usize] = (i, j);
                }
                if 'S' == board[i][j] {
                    cheese_list[0] = (i, j);
                }
            }
        }
        cheese_list
    };

    let graph = {
        let mut graph = Graph::new(h, w);
        let v_list: Vec<(isize, isize)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        for i in 0..h {
            for j in 0..w {
                if board[i][j] == 'X' {
                    continue;
                }
                for &v in &v_list {
                    let (dy, dx) = (i as isize + v.0, j as isize + v.1);
                    if 0 <= dy
                        && dy < h as isize
                        && 0 <= dx
                        && dx < w as isize
                        && board[dy as usize][dx as usize] != 'X'
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

    // Solve
    let ans: usize = solve(&graph, &cheese_list);

    // Output
    println!("{}", ans);
}
