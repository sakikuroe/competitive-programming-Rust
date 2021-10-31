use proconio::input;

fn main() {
        // Input
        input! {
                k: usize,
                a: String, b: String,
        }

        // Solve
        let a = usize::from_str_radix(&a, k as u32).unwrap();
        let b = usize::from_str_radix(&b, k as u32).unwrap();
        let ans = a*b;

        // Output
        println!("{}", ans);
}
