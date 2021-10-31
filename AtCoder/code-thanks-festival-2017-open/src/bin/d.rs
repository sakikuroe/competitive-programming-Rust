use num::integer::gcd;
use proconio::input;

fn main() {
        // Input
        input! {
                n: usize, m: usize,
        }

        // Solve
        let ans = m - gcd(n, m);

        // Output
        println!("{}", ans);
}
