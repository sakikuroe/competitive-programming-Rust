use proconio::{input, marker::Chars};

fn main() {
    // Input
    input! {
        m: usize, n: usize,
        k: usize,
        board: [Chars; m],
        q: [(usize, usize, usize, usize); k],
    }

    // Solve
    let s = {
        let mut res = vec![vec![[0; 3]; n + 1]; m + 1];
        for i in 0..m {
            for j in 0..n {
                match board[i][j] {
                    'J' => res[i + 1][j + 1][0] += 1,
                    'O' => res[i + 1][j + 1][1] += 1,
                    'I' => res[i + 1][j + 1][2] += 1,
                    _ => (),
                }
            }
        }
        for i in 0..m {
            for j in 0..n {
                for k in 0..3 {
                    res[i + 1][j + 1][k] += res[i][j + 1][k] + res[i + 1][j][k] - res[i][j][k];
                }
            }
        }
        res
    };

    for v in q {
        let mut ans = vec![];
        for i in 0..3 {
            ans.push(s[v.2][v.3][i] - s[v.2][v.1 - 1][i] - s[v.0 - 1][v.3][i] + s[v.0 - 1][v.1 - 1][i]);
        }
        println!("{} {} {}", ans[0], ans[1], ans[2]);
    }
}