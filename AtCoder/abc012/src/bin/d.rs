use std::isize::MAX;
use proconio::input;

const INF: isize = MAX;

trait Matrix {
    fn is_negative_cycle(&self) -> bool;
    fn warshall_floyd(&mut self);
}

impl Matrix for Vec<Vec<isize>> {
    fn is_negative_cycle(&self) -> bool {
        for i in 0..self.len() {
            for j in i + 1..self.len() {
                if self[i][j] != INF && self[j][i] != INF {
                    if self[i][j] + self[j][i] < 0 {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn warshall_floyd(&mut self) {
        for k in 0..self.len() {
            for i in 0..self.len() {
                for j in 0..self.len() {
                    if self[i][k] != INF
                        && self[k][j] != INF
                        && self[i][j] > self[i][k] + self[k][j]
                    {
                        self[i][j] = self[i][k] + self[k][j];
                    }
                }
            }
        }
    }
}

fn output(ans: isize) {
    println!("{}", ans);
}

fn solve(mat: &mut Vec<Vec<isize>>) -> isize {
    mat.warshall_floyd();

    *(mat.iter().map(|row| *row.iter().max().unwrap()).collect::<Vec<isize>>()).iter().min().unwrap()
}

fn main() {
    // Input
    input! {
        n: usize,
        m: usize,
        abt_list : [(usize, usize, isize); m],
    }

    // Initialize
    let mut mat: Vec<Vec<isize>> = vec![vec![INF; n]; n];
    for i in 0..n {
        mat[i][i] = 0;
    }
    for (a, b, t) in abt_list {
        mat[a - 1][b - 1] = t;
        mat[b - 1][a - 1] = t;
    }

    // Solve
    let ans = solve(&mut mat);

    // Output
    output(ans);
}