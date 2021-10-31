use proconio::input;

fn main() {
        // Input
        input! {
                h: usize, w: usize,
                n: usize,
                a_list: [usize; n],
        }

        // Solve
        let ans = {
                let mut board = vec![];
                for i in 1..=n {
                        for _ in 0..a_list[i - 1] {
                                board.push(i);
                        }
                }

                let mut res = vec![vec![0; w]; h];
                for i in 0..h {
                        for j in 0..w {
                                if i % 2 == 0 {
                                        res[i][j] = board[i * w + j];
                                } else {
                                        res[i][j] = board[i * w + (w - 1 - j)];
                                }
                        }
                }

                res
        };

        // Output
        for i in 0..h {
                for j in 0..w {
                        print!("{} ", ans[i][j]);
                }
                println!();
        }
}
