use proconio::input;
use superslice::*;

fn main() {
        // Input
        input! {
            n: usize, q: usize,
            mut a_list: [usize; n],
            k_list: [usize; q],
        }

        // Initialize
        a_list.sort();
        // let a_list = {
        //         let mut res = vec![0 as usize];
        //         res.append(&mut a_list);
        //         res
        // };

        // Solve
        let c_list = {
                let mut res = vec![];
                for i in 0..a_list.len() {
                        res.push(a_list[i] - (i + 1));
                }
                res
        };

        let ans_list: Vec<usize> = {
                let mut res = vec![];
                for k in k_list {
                        if c_list.lower_bound(&k) == n {
                                res.push(k + n);
                        } else {
                                res.push(a_list[c_list.lower_bound(&k)] + k
                                        - c_list[c_list.lower_bound(&k)]
                                        - 1);
                        }
                }
                res
        };

        // Output
        for ans in ans_list {
                println!("{}", ans);
        }
}
