use proconio::input;
use std::cmp;

fn main() {
    // Input
    input! {
        n: usize,
        a_list :[usize; n],
    }
    
    // Initialize
    // sum_list[i] = [a_list[0], a_list[j])の総和
    let sum_list = {
        let mut sum_list : Vec<usize> = vec![0;n+1];
        for i in 0..n {
            sum_list[i + 1] = sum_list[i] + a_list[i];
        }

        sum_list
    };
    
    // Solve
    //ans_list := [1, k]における答えのリスト
    let ans_list:Vec<usize> = {
        let mut ans_list:Vec<usize> =vec![];
        for k in 1..=n {
            let ans:usize = {
                let mut sum:usize=0;
                for j in 0..n - (k - 1){
                    sum = cmp::max(sum,sum_list[j+k] - sum_list[j]);
                }

                sum
            };
            ans_list.push(ans);
        }
        
        ans_list
    };
    
    // Output
    for ans in ans_list {
        println!("{}", ans);
    }
}
