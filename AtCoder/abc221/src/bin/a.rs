use proconio::input;

fn main() {
        // Input
        input! {
                a: u32, b: u32,
        }

        // Output
        println!("{}", (32u32).pow(a - b));
}
