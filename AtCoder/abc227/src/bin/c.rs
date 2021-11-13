use proconio::input;

fn main() {
        // Input
        input! {
            n: usize,
        }

        // Initialize
        let mut ans = 0;
        for a in 1..4700 {
                for b in a..=((n / a) as f64).sqrt() as usize {
                        if n / a*b >= b {
                                ans += n / (a*b) - (b - 1);
                                // println!("{} {} {}", a, b, n / (a*b));
                                // println!("{}", ans);
                        }
                }

        }
        println!("{}", ans);
        // Solve

        // Output
}
