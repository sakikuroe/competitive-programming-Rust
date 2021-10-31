use proconio::input;

fn main() {
        // Input
        input! {
                x: usize,
                y: usize,
                z: usize,
        }

        // Initialize

        // Solve
        let ans = if y * z % x == 0 {
                y * z / x - 1
        } else {
                y * z / x
        };

        // Output
        println!("{}", ans);
}
