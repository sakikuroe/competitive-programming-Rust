use proconio::input;

fn main() {
        // Input
        input! {
                n: usize,
        }
        
        // Solve
        let a = {
                let mut d = 0;
                for i in 1..=(n as f64).sqrt() as usize {
                        if n % i == 0 {
                                 d = i;
                        }
                }
                n / d
        };
        
        let ans = {
                (a as f64).log10() as usize  + 1
        };

        // Output
        println!("{}", ans);
}
