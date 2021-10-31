use memoise::memoise;
use proconio::{input, marker::Chars};
use std::{cmp, usize::MAX};

const INF: usize = MAX;

#[memoise (0 <= n <= 998, 0 <= c <= 2)]
// dp(n - 1, c) := n列目(1 <= n <= 999)がc色となるように塗るときの、塗り替える必要のあるタイルの枚数の最小値
// c <=> 0: R, 1: B, 2: W
fn dp(n: usize, c: usize, s_mat: &Vec<Vec<char>>) -> usize {
    let color = {
        match c {
            0 => 'R',
            1 => 'B',
            2 => 'W',
            _ => ' ',
        }
    };

    // n - 1列目のすべてのタイルをc色にするのに塗り替える必要のあるタイルの枚数
    let change: usize = {
        let mut change: usize = 0;
        for i in 0..5 {
            if color != s_mat[i][n] {   
                change += 1;
            }
        }
        
        change
    };


    // 初期条件
    if n == 0 {
        return change;
    } 
        
    let res: usize = {
        let mut res: usize = INF;

        for (i, &x) in vec!['R', 'B', 'W'].iter().enumerate() {
            if color == x {
                continue;
            }
            res = cmp::min(res, dp(n - 1, i, s_mat) + change);
        }

        res
    };

    res
}

fn solve(n: usize, s_mat: &Vec<Vec<char>>) -> usize {
    let res: usize = {
        let mut res: usize = INF;
        for i in 0..3 {
            res = cmp::min(res, dp(n - 1, i, s_mat));
        }

        res
    };

    res
}

fn main() {
    // Input
    input! {
        n: usize,
        s_mat: [Chars; 5],
    }

    // Solve
    let ans: usize = solve(n, &s_mat);

    // Output
    println!("{}", ans);
}
