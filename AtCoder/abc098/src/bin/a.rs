use proconio::input;
use std::cmp::max;

fn main() {
        // Input
        input! {
                a: isize, b: isize,
        }
    
        // Initialize
        
        // Solve
        let ans = max(a+b,max(a-b,a*b));
        
        // Output
        println!("{}", ans);
}
