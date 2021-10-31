use proconio::input;

fn main() {
        // Input
        input! {
                h: usize, w: usize,
                a_mat: [[usize;w];h],
        }

        // Initialize

        // Solve
        let ans = {
                let mut f = true;
                for i1 in 0..h {
                        for i2 in i1 + 1..h {
                                for j1 in 0..w {
                                        for j2 in j1 + 1..w {
                                                if a_mat[i1][j1] + a_mat[i2][j2]
                                                        > a_mat[i2][j1] + a_mat[i1][j2]
                                                {
                                                        f = false;
                                                }
                                        }
                                }
                        }
                }

                if f {
                        "Yes"
                } else {
                        "No"
                }
        };

        // Output
        println!("{}", ans);
}
