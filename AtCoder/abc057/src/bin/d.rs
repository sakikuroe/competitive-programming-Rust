use proconio::input;
use std::cmp;

fn main() {
        // Input
        input! {
                n: usize, a: usize, b: usize,
                mut v_list: [usize; n],
        }
    
        // Initialize
        let comb = {
                let mut comb = vec![vec![1 as usize; 51]; 51];
                for i in 1..=50 {
                        for j in 1..=i-1 {
                                comb[i][j] = comb[i - 1][j- 1]+comb[i-1][j];
                        }
                }
                comb
        };

        v_list.sort();
        v_list.reverse();
        
        // Solve
        let average = {
                let mut sum = 0.0;
                for i in 0..a {
                        sum += v_list[i] as f64;
                }
                sum / (a as f64)
        };

        let n = {
                if v_list[0] == v_list[a-1] {
                        let mut res = 0;
                        let x = v_list[0];
                        let n = v_list.clone().into_iter().filter(|&y| y == x).collect::<Vec<usize>>().len();
                        for i in a..=cmp::min(b, n) {
                                res += comb[n][i];
                        }
                        res

                } else {
                        let x = v_list[a-1];
                        let n = v_list.clone().into_iter().filter(|&y| y == x).collect::<Vec<usize>>().len();
                        let r = {
                                let mut sum = 0;
                                for i in 0..a {
                                        if v_list[i] == x {
                                                sum += 1;
                                        }
                                }
                                sum
                        };
                        comb[n][r]
                }
        };
        
        // Output
        println!("{}", average);
        println!("{}", n);
}
