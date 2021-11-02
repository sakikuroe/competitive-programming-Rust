use std::io;

fn dfs(sh: usize, sw: usize, h: usize, w: usize, c_mat: &mut Vec<Vec<usize>>) {
    c_mat[sh][sw] = 0;
    let vy: Vec<isize> = vec![0, 1, 1, 1, 0, -1, -1, -1];
    let vx: Vec<isize> = vec![1, 1, 0, -1, -1, -1, 0, 1];

    for i in 0..8 {
        let (dh, dw) = (sh as isize + vy[i], sw as isize + vx[i]);
        if 0 <= dh
            && dh < h as isize
            && 0 <= dw
            && dw < w as isize
            && c_mat[dh as usize][dw as usize] == 1
        {
            dfs(dh as usize, dw as usize, h, w, c_mat)
        }
    }
}

fn main() {
    loop {
        // Input
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let word_list: Vec<&str> = line.split_whitespace().collect();
        let w: usize = word_list[0].parse::<usize>().unwrap();
        let h: usize = word_list[1].parse::<usize>().unwrap();
        if w == 0 && h == 0 {
            break;
        }
        let mut c_mat: Vec<Vec<usize>> = {
            let mut res: Vec<Vec<usize>> = vec![vec![]; h];
            for i in 0..h {
                line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                let word_list: Vec<usize> = {
                    let word_list: Vec<&str> = line.split_whitespace().collect();
                    word_list
                        .into_iter()
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect()
                };
                res[i] = word_list;
            }
            res
        };


        // Solve
        // c_mat[i][j]が陸地だったら(i, j)からdfs
        // dfsで陸地を潰して海にしていく
        let mut cnt = 0;
        for i in 0..h {
            for j in 0..w {
                if c_mat[i][j] == 1 {
                    cnt += 1;
                    dfs(i, j, h, w, &mut c_mat);
                }
            }
        }

        // Output
        println!("{}", cnt);
    }
}
