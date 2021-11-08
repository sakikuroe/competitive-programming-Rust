use proconio::input;

fn main() {
        // Input
        input! {
            x: f64,
        }

        // Solve
        let ans;
        if x % 1.00 >= 0.5 {
                ans = x as usize + 1;
        } else {
                ans = x as usize;
        }

        // Output
        println!("{}", ans);
}
