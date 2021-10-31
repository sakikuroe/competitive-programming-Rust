use proconio::input;
use std::usize::MAX;

const INF: usize = MAX;

trait Matrix {
    fn min_distance_sum(&mut self) -> isize;
}

impl Matrix for Vec<Vec<usize>> {
    fn min_distance_sum(&mut self) -> isize {
        let mut res: usize = 0;
        for k in 0..self.len() {
            for i in 0..self.len() {
                for j in 0..self.len() {
                    if self[i][k] != INF && self[k][j] != INF {
                        if self[i][j] != INF && self[i][j] > self[i][k] + self[k][j] {
                            return -1;
                        } else if  k != i && i != j && j != k && self[i][j] == self[i][k] + self[k][j] {
                            self[i][j] = INF;
                            self[j][i] = INF;
                        }
                    }
                }
            }
        }

        for i in 0..self.len() {
            for j in (i + 1)..self.len() {
                if self[i][j] != INF {
                    res += self[i][j];
                }
            }
        }

        res as isize
    }
}

fn solve(matrix: &mut Vec<Vec<usize>>) -> isize {
    matrix.min_distance_sum()
}

fn output(ans: isize) {
    println!("{}", ans);
}

fn main() {
    // Input
    input! {
        n: usize,
        mut a_matrix: [[usize; n]; n],
    }

    // Solve
    let ans: isize = solve(&mut a_matrix);

    // Output
    output(ans);
}
