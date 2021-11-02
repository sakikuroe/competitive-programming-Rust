use std::{io, isize::MAX};
// use proconio::input;

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

fn output(mat: &mut Vec<Vec<isize>>) {
    if mat.is_negative_cycle() {
        println!("NEGATIVE CYCLE");
    } else {
        for row in mat {
            for (i, &num) in row.clone().iter().enumerate() {
                if num == INF {
                    print!("INF");
                } else {
                    print!("{}", num);
                }
                if i as usize != row.len() - 1 {
                    print!(" ");
                }
            }
            println!("");
        }
    }
}

fn solve(mat: &mut Vec<Vec<isize>>) {
    mat.warshall_floyd();
}

fn main() {
    // Input
    // input! {
    //     v_size: usize,
    //     e_size: usize,
    //     std_list: [(usize, usize, isize); e_size],
    // }
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let word_list: Vec<&str> = line.split_whitespace().collect();
    let v_size: usize = word_list[0].parse::<usize>().unwrap();
    let e_size: usize = word_list[1].parse::<usize>().unwrap();

    let mut std_list: Vec<(usize, usize, isize)> = Vec::new();
    for _ in 0..e_size {
        line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let word_list: Vec<isize> = {
            let word_list: Vec<&str> = line.split_whitespace().collect();
            word_list
                .into_iter()
                .map(|s| s.parse::<isize>().unwrap())
                .collect()
        };
        std_list.push((word_list[0] as usize, word_list[1] as usize, word_list[2]));
    }

    // Initialize
    let mut mat: Vec<Vec<isize>> = vec![vec![INF; v_size]; v_size];
    for i in 0..v_size {
        mat[i][i] = 0;
    }
    for (s, t, d) in std_list {
        mat[s][t] = d;
    }

    // Solve
    solve(&mut mat);

    // Output
    output(&mut mat);
}
