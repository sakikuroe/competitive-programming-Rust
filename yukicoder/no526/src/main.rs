use std::io;

fn mat_mul(mat1: &Vec<Vec<usize>>, mat2: &Vec<Vec<usize>>, m: usize) -> Vec<Vec<usize>> {
        let mut res = vec![vec![0, 0], vec![0, 0]];
        for i in 0..2 {
                for j in 0..2 {
                        res[i][j] = (mat1[i][0] % m) * (mat2[0][j] % m)
                                + (mat1[i][1] % m) * (mat2[1][j] % m);
                        res[i][j] %= m;
                }
        }

        res
}

fn mat_pow(mat: &Vec<Vec<usize>>, n: usize, m: usize) -> Vec<Vec<usize>> {
        if n == 0 {
                vec![vec![1, 0], vec![0, 1]]
        } else if n % 2 == 0 {
                mat_pow(&mat_mul(mat, mat, m), n / 2, m)
        } else {
                mat_mul(mat, &mat_pow(mat, n - 1, m), m)
        }
}

fn main() {
        let (n, m) = {
                let mut line = String::new();
                io::stdin().read_line(&mut line).unwrap();
                let word_list: Vec<&str> = line.split_whitespace().collect();
                let n: usize = word_list[0].parse::<usize>().unwrap();
                let m: usize = word_list[1].parse::<usize>().unwrap();
                (n, m)
        };

        let mat = vec![vec![1, 1], vec![1, 0]];
        let ans = mat_pow(&mat, n - 2, m)[0][0] % m;

        println!("{}", ans);
}
