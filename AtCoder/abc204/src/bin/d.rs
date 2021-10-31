use proconio::input;

fn main() {
    // Input
    input! {
        n: usize,
        t_list: [usize; n],
    }
    
    // Solve
    let sum = {
        let mut res = 0;
        for &t in &t_list {
            res += t;
        }
        res
    };
    let mut ans: usize = 1000000000;

    for t in t_list{
        if (sum as isize) - dp()
    }

    
    // Output

}
