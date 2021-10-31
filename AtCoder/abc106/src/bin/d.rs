use proconio::input;

fn main() {
    // Input
    input! {
        n: usize, m: usize, q: usize,
        lr_list: [(usize, usize); m],
        pq_list: [(usize, usize); q],
    }

    // Initialize
    let sum_mat: Vec<Vec<usize>> = {
        let mut sum_mat: Vec<Vec<usize>> = vec![vec![0; n + 1]; n + 1];
        for (l, r) in lr_list {
            sum_mat[l][r] += 1;
        }

        println!();
        for i in 0..n {
            for j in 0..n {
                sum_mat[i + 1][j + 1] += sum_mat[i][j + 1] + sum_mat[i + 1][j] - sum_mat[i][j];
            }
        }
        sum_mat
    };

    // Solve
    let ans_list: Vec<usize> = {
        let mut res: Vec<usize> = vec![];
        for (p, q) in pq_list {
            res.push(sum_mat[q][q] + sum_mat[p - 1][p - 1] - sum_mat[q][p - 1] - sum_mat[p - 1][q]);
        }
        res
    };

    // Output
    for ans in ans_list {
        println!("{}", ans);
    }
}
