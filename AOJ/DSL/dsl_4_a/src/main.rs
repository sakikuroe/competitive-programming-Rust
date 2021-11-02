// use proconio::input;
use std::io;

fn compress(v1: &mut Vec<isize>, v2: &mut Vec<isize>) -> Vec<isize> {
        // vs: 目盛り
        let mut vs: Vec<isize> = vec![];
        for v in v1.into_iter() {
                vs.push(*v);
                vs.push(*v + 1);
        }
        for v in v2.into_iter() {
                vs.push(*v);
                vs.push(*v + 1);
        }
        vs.sort();
        vs.dedup();

        for v in v1.into_iter() {
                *v = vs.binary_search(v).unwrap() as isize;
        }
        for v in v2.into_iter() {
                *v = vs.binary_search(v).unwrap() as isize;
        }

        vs
}

fn main() {
        // Input
        // input! {
        //         n: usize,
        //         xyxy_list: [(isize, isize, isize, isize); n],
        // }
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let word_list: Vec<&str> = line.split_whitespace().collect();
        let n: usize = word_list[0].parse::<usize>().unwrap();

        let mut xyxy_list: Vec<(isize, isize, isize, isize)> = Vec::new();
        for _ in 0..n {
                line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                let word_list: Vec<isize> = {
                        let word_list: Vec<&str> = line.split_whitespace().collect();
                        word_list.into_iter().map(|s| s.parse::<isize>().unwrap()).collect()
                };
                xyxy_list.push((word_list[0], word_list[1], word_list[2], word_list[3]));
        }

        // Initialize
        let mut x1_list: Vec<isize> = vec![];
        let mut y1_list: Vec<isize> = vec![];
        let mut x2_list: Vec<isize> = vec![];
        let mut y2_list: Vec<isize> = vec![];

        for &(x1, y1, x2, y2) in &xyxy_list {
                x1_list.push(x1);
                y1_list.push(y1);
                x2_list.push(x2);
                y2_list.push(y2);
        }

        // Solve
        // 目盛り
        let xs: Vec<isize> = compress(&mut x1_list, &mut x2_list);
        let ys: Vec<isize> = compress(&mut y1_list, &mut y2_list);

        let mut imos_mat: Vec<Vec<i16>> = vec![vec![0; ys.len()]; xs.len()];

        // いもす法
        // +1, -1の記録
        for i in 0..n {
                imos_mat[x1_list[i] as usize][y1_list[i] as usize] += 1;
                imos_mat[x2_list[i] as usize][y2_list[i] as usize] += 1;
                imos_mat[x1_list[i] as usize][y2_list[i] as usize] -= 1;
                imos_mat[x2_list[i] as usize][y1_list[i] as usize] -= 1;
        }

        // シミュレーション
        for i in 0..xs.len() {
                for j in 0..ys.len() - 1 {
                        imos_mat[i][j + 1] += imos_mat[i][j];
                }
        }
        for i in 0..xs.len() - 1 {
                for j in 0..ys.len() {
                        imos_mat[i + 1][j] += imos_mat[i][j];
                }
        }

        // 各区画について、大きさを足し合わせる
        let ans: isize = {
                let mut ans: isize = 0;
                for i in 0..xs.len() {
                        for j in 0..ys.len() {
                                if imos_mat[i][j] > 0 {
                                        ans += (xs[i + 1] - xs[i]) * (ys[j + 1] - ys[j]);
                                }
                        }
                }

                ans
        };

        // Output
        println!("{}", ans);
}
