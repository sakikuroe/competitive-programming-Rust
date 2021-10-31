use proconio::input;

fn main() {
        // Input
        input! {
                x: usize,
        }

        // Solve
        let ans = if x < 1200 {
                "ABC"
        } else {
                "ARC"
        };

        // Output
        println!("{}", ans);
}
