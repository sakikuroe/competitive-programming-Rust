use proconio::input;
use std::ops;
use std::usize::MAX;

#[derive(Clone, Debug)]
struct Matrix {
        rows: usize,
        cols: usize,
        data: Vec<Vec<usize>>,
}
impl Matrix {
        fn new(data: Vec<Vec<usize>>) -> Matrix {
                Matrix {
                        rows: data.len(),
                        cols: data[0].len(),
                        data,
                }
        }
        fn mat_pow(&self, n: usize) -> Result<Matrix, &str> {
                if self.rows != self.cols {
                        return Err("Because it is not a square matrix, matrix power cannot be defined.");
                }
                if n == 0 {
                        Ok(Matrix {
                                rows: self.rows,
                                cols: self.cols,
                                data: {
                                        let mut res = vec![vec![0; self.cols]; self.rows];
                                        for i in 0..self.rows {
                                                res[i][i] = MAX;
                                        }
                                        res
                                },
                        })
                } else if n % 2 == 0 {
                        Ok((self.clone() * self.clone()).unwrap().mat_pow(n / 2).unwrap())
                } else {
                        Ok((self.clone() * self.clone().mat_pow(n - 1).unwrap()).unwrap())
                }
        }
}
impl ops::Mul for Matrix {
        type Output = Result<Matrix, &'static str>;

        fn mul(self, other: Self) -> Result<Matrix, &'static str> {
                if self.cols != other.rows {
                        return Err("Because the number of columns in the matrix on the left and the number of rows in the matrix on the right are different, 
                        the product of the matrices cannot be calculated.");
                }

                let mut data = vec![vec![0; other.cols]; self.rows];

                for i in 0..self.rows {
                        for j in 0..other.cols {
                                for k in 0..self.rows {
                                        data[i][j] ^= self.data[i][k] & other.data[k][j];
                                }
                        }
                }
                Ok(Self {
                        rows: self.rows,
                        cols: other.cols,
                        data,
                })
        }
}

fn main() {
        // Input
        input! {
                k: usize,
                m: usize,
                a_list: [usize; k],
                c_list: [usize; k],
        }

        // Solve
        let coefficient_mat = {
                let mut mat: Vec<Vec<usize>> = vec![vec![0; k]; k];
                for i in 0..k {
                        mat[0][i] = c_list[i];
                }
                for i in 0..k - 1 {
                        mat[i + 1][i] = MAX;
                }
                Matrix::new(mat)
        };

        let x = {
                let mut x = vec![vec![0; 1]; k];
                for i in 0..k {
                        x[i][0] = a_list[k - 1 - i];
                }
                Matrix::new(x)
        };

        let ans = if m > k {
                ((coefficient_mat.mat_pow(m - k).unwrap()) * x).unwrap().data[0][0]
        } else {
                a_list[m - 1]
        };

        // Output
        println!("{:?}", ans);
}
