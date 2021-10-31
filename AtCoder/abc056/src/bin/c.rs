use proconio::input;

fn main() {
        // Input
        input! {
                x: usize,
        }
        
        // Solve
        let ans = {
                let mut res = 0;

                for i in 1..100000 {
                        if i * (i + 1) / 2 >= x {
                                res = i;
                                break;
                        }
                }
                
                res
        };
        
        // Output
        println!("{}", ans);
}
