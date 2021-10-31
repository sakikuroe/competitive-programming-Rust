use proconio::input;

fn main() {
        // Input
        input! {
            r: usize, x: usize, y: usize,
        }

        // Initialize
        let d = (x as f64 * x as f64 + y as f64 * y as f64).sqrt();

        // Solve
        let ans = {
                if d < r as f64 {
                        2
                } else {
                        let mut res = 0;
                        let mut sum = 0.0;
                        while sum < d {
                                res += 1;
                                sum += r as f64;
                        }

                        res
                }
        };

        // Output
        println!("{}", ans);
}
