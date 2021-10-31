use proconio::input;
use std::cmp;

fn main() {
        // Input
        input! {
                n: usize,
                a_list: [isize; n],
        }

        // Solve
        let x = {
                let mut sum = 0;
                let mut b_list: Vec<isize> = vec![];

                for i in 0..n {
                        sum += a_list[i];
                        if i % 2 == 0 {
                                if sum <= 0 {
                                        b_list.push(1 - sum + a_list[i]);
                                        sum = 1;
                                } else {
                                        b_list.push(a_list[i])
                                }
                        } else {
                                if sum >= 0 {
                                        b_list.push(-1 - sum + a_list[i]);
                                        sum = -1;
                                } else {
                                        b_list.push(a_list[i])
                                }
                        }
                }

                a_list.iter().zip(b_list).map(|(&a, b)| (a - b).abs()).sum::<isize>()
        };
        let y = {
                let mut sum = 0;
                let mut b_list: Vec<isize> = vec![];

                for i in 0..n {
                        sum += a_list[i];
                        if i % 2 == 1 {
                                if sum <= 0 {
                                        b_list.push(1 - sum + a_list[i]);
                                        sum = 1;
                                } else {
                                        b_list.push(a_list[i])
                                }
                        } else {
                                if sum >= 0 {
                                        b_list.push(-1 - sum + a_list[i]);
                                        sum = -1;
                                } else {
                                        b_list.push(a_list[i])
                                }
                        }
                }

                a_list.iter().zip(b_list).map(|(&a, b)| (a - b).abs()).sum::<isize>()
        };

        let ans = cmp::min(x, y);

        // Output
        println!("{}", ans);
}
