use proconio::input;

fn main() {
    // Input
    input! {
        a: usize, b: usize, k: usize,
    }

    // Solve
    let (a, b) = if k <= a {
        (a - k, b)
    } else if k - a <= b {
        (0, b - (k - a))
    } else {
        (0, 0)
    };

    // Output
    println!("{} {}", a, b);
}
