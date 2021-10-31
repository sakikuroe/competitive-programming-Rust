use proconio::input;

fn main() {
        // Input
        input! {
                x: usize,
        }

        // Initialize

        // Solve
        let ans = if x >= 100 && x % 100 == 0 {
                "Yes"
        } else {
                "No"
        };

        // Output
        println!("{}", ans);
}
