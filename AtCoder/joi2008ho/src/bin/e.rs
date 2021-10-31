use proconio::input;
use std::{collections::VecDeque, vec};

fn bfs(sh: usize, sw: usize, h: usize, w: usize, board: &mut Vec<Vec<bool>>) {
        let mut que: VecDeque<(u16, u16)> = VecDeque::new();
        que.push_back((sh as u16, sw as u16));
        board[sh][sw] = true;
        let vy: Vec<isize> = vec![0, 1, 0, -1];
        let vx: Vec<isize> = vec![1, 0, -1, 0];

        while let Some((current_y, current_x)) = que.pop_front() {
                for i in 0..4 {
                        let (dh, dw) = (current_y as isize + vy[i], current_x as isize + vx[i]);
                        if 0 <= dh
                                && dh < h as isize
                                && 0 <= dw
                                && dw < w as isize
                                && board[dh as usize][dw as usize] == false
                        {
                                board[dh as usize][dw as usize] = true;
                                que.push_back((dh as u16, dw as u16));
                        }
                }
        }
}

fn compress(v1: &mut Vec<u32>, v2: &mut Vec<u32>, size: usize) -> usize {
        let mut vs: Vec<u32> = vec![];
        for a in v1.into_iter() {
                for direct in vec![-1, 0, 1] {
                        if 0 <= (*a) as i32 + direct && (*a) as i32 + direct <= size as i32 {
                                vs.push(((*a) as i32 + direct) as u32);
                        }
                }
        }
        for a in v2.into_iter() {
                for direct in vec![-1, 0, 1] {
                        if 0 <= (*a) as i32 + direct && (*a) as i32 + direct <= size as i32 {
                                vs.push(((*a) as i32 + direct) as u32);
                        }
                }
        }
        vs.sort();
        vs.dedup();
        for a in v1.into_iter() {
                *a = vs.binary_search(a).unwrap() as u32;
        }
        for a in v2.into_iter() {
                *a = vs.binary_search(a).unwrap() as u32;
        }
        vs.len() as usize
}

fn main() {
        // Input
        input! {
            w: usize, h: usize,
            n: usize,
        }
        // Initialize
        let mut x1_list: Vec<u32> = vec![];
        let mut y1_list: Vec<u32> = vec![];
        let mut x2_list: Vec<u32> = vec![];
        let mut y2_list: Vec<u32> = vec![];
        for _ in 0..n {
                input! {
                    (x1, y1, x2, y2): (u32, u32, u32, u32),
                }
                x1_list.push(x1);
                y1_list.push(y1);
                x2_list.push(x2);
                y2_list.push(y2);
        }


        let xs_len: usize = compress(&mut x1_list, &mut x2_list, w);
        let ys_len: usize = compress(&mut y1_list, &mut y2_list, h);

        // いもす法
        let mut imos: Vec<Vec<i16>> = vec![vec![0; ys_len]; xs_len];
        // +1, -1の記録
        for i in 0..n {
                imos[x1_list[i] as usize][y1_list[i] as usize] += 1;
                imos[x2_list[i] as usize][y2_list[i] as usize] += 1;
                imos[x1_list[i] as usize][y2_list[i] as usize] -= 1;
                imos[x2_list[i] as usize][y1_list[i] as usize] -= 1;
        }

        // シミュレーション
        for i in 0..xs_len - 1 {
                for j in 0..ys_len {
                        imos[i + 1][j] += imos[i][j];
                }
        }
        for i in 0..xs_len {
                for j in 0..ys_len - 1 {
                        imos[i][j + 1] += imos[i][j];
                }
        }

        let mut compressed_board: Vec<Vec<bool>> = {
                let mut compressed_board = vec![vec![false; ys_len - 1]; xs_len - 1];
                for i in 0..xs_len - 1 {
                        for j in 0..ys_len - 1 {
                                if imos[i][j] > 0 {
                                        compressed_board[i][j] = true;
                                }
                        }
                }
                compressed_board
        };

        // bfsで連結な0の領域の個数を求める
        let mut cnt: usize = 0;
        for i in 0..compressed_board.len() {
                for j in 0..compressed_board[0].len() {
                        if compressed_board[i][j] == false {
                                cnt += 1;
                                bfs(
                                        i,
                                        j,
                                        compressed_board.len(),
                                        compressed_board[0].len(),
                                        &mut compressed_board,
                                );
                        }
                }
        }

        // Output
        println!("{}", cnt);
}
