use proconio::input;
use superslice::*;

fn main() {
        // Input
        input! {
                n: usize, w: usize,
                vw_list: [(isize, usize); n],
        }
        
        // Solve
        let first_list = {
                let mut res: Vec<Vec<isize>> = vec![vec![]; n+1];
                for bit in 0..(1 << n / 2) {
                        // m := 荷物の重さ
                        let mut m = 0;
                        // m個の荷物の合計価値
                        let sum: isize = {
                                let mut sum = 0;
                                for i in 0..(n / 2) {
                                        if bit & (1 << i) != 0 {
                                                sum += vw_list[i].0;
                                                m += vw_list[i].1;
                                        }
                                }
                                sum
                        };
                        res[m].push(sum);
                }

                res.into_iter()
                        .map(|mut v| {
                                v.sort();
                                v
                        })
                        .collect::<Vec<Vec<isize>>>()
        };

        let second_list = {
                let mut res: Vec<Vec<isize>> = vec![vec![]; n+1];
                for bit in 0..(1 << n - n / 2) {
                        // m := 荷物の重さ
                        let mut m = 0;
                        // m個の荷物の合計価値
                        let sum: isize = {
                                let mut sum = 0;
                                for i in 0..(n - n / 2) {
                                        if bit & (1 << i) != 0 {
                                                sum += vw_list[i].0;
                                                m += vw_list[i].1;
                                        }
                                }
                                sum
                        };
                        res[m].push(sum);
                }

                res.into_iter()
                        .map(|mut v| {
                                v.sort();
                                v
                        })
                        .collect::<Vec<Vec<isize>>>()
        };
        
        // Output
}
