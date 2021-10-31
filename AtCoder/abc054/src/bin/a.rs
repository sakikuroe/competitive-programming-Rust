use proconio::input;

fn main() {
        // Input
        input! {
                mut a: usize,
                mut b: usize,
        }

        // Initialize
        if a == 1 {
                a = 14;
        }
        if b == 1 {
                b = 14;
        }

        // Solve
        let ans = if a > b {
                "Alice"
        } else if a == b {
                "Draw"
        } else {
                "Bob"
        };

        // Output
        println!("{}", ans);
}
