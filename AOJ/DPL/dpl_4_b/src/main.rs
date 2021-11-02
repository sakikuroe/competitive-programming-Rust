use proconio::input;
use superslice::*;

fn main() {
        // Input
        input! {
                n: usize, k: usize, l: isize, r: isize,
                mut a_list: [usize; n],
        }

        // Initialize
        // first_list[i] = 前半分のうちのi枚のコインの合計によって実現可能な金額の集合
        let first_list = {
                let mut res: Vec<Vec<isize>> = vec![vec![]; n+1];
                for bit in 0..(1 << n / 2) {
                        // m := コインの数(bitの立っている本数)
                        let m = (bit as u64).count_ones() as usize;
                        // m枚のコインの合計
                        let sum: usize = {
                                let mut sum = 0;
                                for i in 0..(n / 2) {
                                        if bit & (1 << i) != 0 {
                                                sum += a_list[i];
                                        }
                                }
                                sum
                        };
                        res[m].push(sum as isize);
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
                        // m := コインの数(bitの立っている本数)
                        let m = (bit as u64).count_ones() as usize;
                        // m枚のコインの合計
                        let sum: usize = {
                                let mut sum = 0;
                                for i in 0..(n - n / 2) {
                                        if bit & (1 << i) != 0 {
                                                sum += a_list[i + n / 2];
                                        }
                                }
                                sum
                        };
                        res[m].push(sum as isize);
                }

                res.into_iter()
                        .map(|mut v| {
                                v.sort();
                                v
                        })
                        .collect::<Vec<Vec<isize>>>()
        };

        // Solve
        let mut sum = 0;
        for i in 0..=k {  
                let first = first_list[i].clone();
                let second = second_list[k-i].clone();
                for a in first {
                        // sum += (r-a 以上 l-a 以下の個数
                        sum += second.upper_bound(&(r-a)) - second.lower_bound(&(l-a))
                }
        
        }

        // Output
        println!("{}", sum);
}
