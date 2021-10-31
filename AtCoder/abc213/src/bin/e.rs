use proconio::{input, marker::Chars};
use std::isize::MAX;
use std::{
        cmp::{Ordering, Reverse},
        collections::BinaryHeap,
};

const INF: isize = MAX;

#[derive(Eq)]
struct Node {
        vertex: (isize, isize),
        priory: Reverse<isize>,
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

fn main() {
        // Input
        input! {
                h: isize, w: isize,
                s_mat: [Chars; h],
        }

        let mut dist_mat: Vec<Vec<isize>> = vec![vec![INF; w as usize]; h as usize];
        let mut visited_mat: Vec<Vec<bool>> = vec![vec![false; w as usize]; h as usize];
        // Solve
        // Dijkstra法
        let mut que: BinaryHeap<Node> = {
                let mut que = BinaryHeap::new();
                que.push(Node {
                        vertex: (0, 0),
                        priory: Reverse(0),
                });
                dist_mat[0][0] = 0;
                que
        };

        while let Some(Node {
                vertex: (i, j),
                priory: _,
        }) = que.pop()
        {
                if visited_mat[i as usize][j as usize] {
                        continue;
                };
                for vx in -1..=1 as isize {
                        for vy in -1..=1 as isize {
                                if vx.abs() + vy.abs() == 2 {
                                        continue;
                                };
                                let ny = i + vy;
                                let nx = j + vx;

                                if ny < 0 || h <= ny || nx < 0 || w <= nx {
                                        continue;
                                }
                                if s_mat[ny as usize][nx as usize] == '#' {
                                        continue;
                                }

                                if dist_mat[ny as usize][nx as usize]
                                        > dist_mat[i as usize][j as usize]
                                {
                                        let dist = dist_mat[i as usize][j as usize];
                                        dist_mat[ny as usize][nx as usize] = dist;
                                        que.push(Node {
                                                vertex: (ny, nx),
                                                priory: Reverse(dist),
                                        });
                                }
                        }
                }
                for vx in -2..=2 as isize {
                        for vy in -2..=2 as isize {
                                if vx.abs() + vy.abs() == 4 {
                                        continue;
                                };
                                let ny = i + vy;
                                let nx = j + vx;

                                if ny < 0 || h <= ny || nx < 0 || w <= nx {
                                        continue;
                                }

                                if dist_mat[ny as usize][nx as usize]
                                        > dist_mat[i as usize][j as usize]
                                {
                                        let dist = dist_mat[i as usize][j as usize] + 1;
                                        dist_mat[ny as usize][nx as usize] = dist;
                                        que.push(Node {
                                                vertex: (ny, nx),
                                                priory: Reverse(dist),
                                        });
                                }
                        }
                }
                visited_mat[i as usize][j as usize] = true;
        }

        let ans = dist_mat[(h - 1) as usize][(w - 1) as usize];

        // Output
        println!("{}", ans);
}
