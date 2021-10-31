use proconio::input;

fn main() {
        // Input
        input! {
                t: usize,
        }

        // Initialize

        // Solve
        for _ in 0..t {
                input! {
                        n: usize,
                }
                if n % 4 == 0 {
                        println!("Even");
                } else if n % 2 == 0 {
                        println!("Same");
                } else {
                        println!("Odd");
                }
        }

        // Output
}
