use proconio::input;

fn main() {
        // Input
        input! {
                n: usize, m: usize,
        }
    
        // Solve
        let ans = {
                if n > m / 2{
                        m / 2
                } else {
                        n + (m - 2 * n) / 4
                }
        };
        
        // Output
        println!("{}", ans);
}
