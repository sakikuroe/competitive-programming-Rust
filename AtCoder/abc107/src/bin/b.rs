use proconio::input;
use proconio::marker::Chars;

fn main() {
        // Input
        input! {
                h: usize, w: usize,
                a_mat: [Chars; h],
        }
    
        // Solve
        let mut f_mat = vec![vec![true; w]; h];
        
        for i in 0..h {
                for j in 0..w {
                        if a_mat[i][j] == '.' {
                                if a_mat[i].iter().fold(true, |x, y| x && *y == '.') {
                                        f_mat[i][j] = false;
                                }
                                if a_mat.iter().fold(true, |x, y| x && y[j] == '.') {
                                        f_mat[i][j] = false;
                                }
                        }
                }
        }

        // Output
        for i in 0..h {
                let mut f = false;
                for j in 0..w {
                        if f_mat[i][j] {
                                print!("{}", a_mat[i][j]);
                                f = true;
                        }
                }
                if f {
                        println!();
                }
        }
}
