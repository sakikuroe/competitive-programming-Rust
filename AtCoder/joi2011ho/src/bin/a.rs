use proconio::input;
use proconio::marker::Chars;

fn main() {
    // Input
    input! {
        m: usize, n: usize,
        k: usize,
        board: [Chars; m],
        q_list: [(usize, usize, usize, usize); k],
    }

    // Initialize
    let sum_mat: Vec<Vec<Vec<usize>>> = {
        let mut sum_mat: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; 3]; n + 1]; m + 1];
        for i in 0..m {
            for j in 0..n {
                for k in 0..3 {
                    sum_mat[i + 1][j + 1][k] =
                        sum_mat[i][j + 1][k] + sum_mat[i + 1][j][k] - sum_mat[i][j][k];
                }

                match board[i][j] {
                    'J' => sum_mat[i + 1][j + 1][0] += 1,
                    'O' => sum_mat[i + 1][j + 1][1] += 1,
                    'I' => sum_mat[i + 1][j + 1][2] += 1,
                    _ => (),
                }
            }
        }
        sum_mat
    };

    // Solve
    let ans_list: Vec<Vec<usize>> = {
        let mut ans_list: Vec<Vec<usize>> = vec![];
        for (a, b, c, d) in q_list {
            let ans: Vec<usize> = {
                let mut ans: Vec<usize> = vec![];
                for i in 0..3 {
                    ans.push(
                        sum_mat[c][d][i] + sum_mat[a - 1][b - 1][i]
                            - sum_mat[c][b - 1][i]
                            - sum_mat[a - 1][d][i],
                    );
                }
                ans
            };
            ans_list.push(ans);
        }
        ans_list
    };

    // Output
    for ans in ans_list {
        println!("{} {} {}", ans[0], ans[1], ans[2]);
    }
}
