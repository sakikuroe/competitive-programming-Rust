use cargo_snippet::snippet;
use std::cmp;
use proconio::input;

#[snippet(name = warshall_floyd, prefix = "use std::cmp;")]
fn warshall_floyd(mat: &mut Vec<Vec<usize>>) {
        for k in 0..mat.len() {
                for i in 0..mat.len() {
                        for j in 0..mat.len() {
                                mat[i][j] = cmp::min(mat[i][j], mat[i][k] + mat[k][j]);
                        }
                }
        }
}

fn main() {
        input! {
                v: usize, e: usize,
                std_list: [(usize, usize, usize); e],
        }
        let mut mat: Vec<Vec<usize>> = vec![vec![10000;v];v];
        for i in 0..v {
                mat[i][i] = 0;
        }
        for (s, t, d) in std_list {
                mat[s][t] = d;
        }

        warshall_floyd(&mut mat);
        for m in &mat {
                println!("{:?}", m);
        }
        println!("Hello, world!");
}
