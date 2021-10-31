use proconio::input;

fn main() {
    // Input
    input! {
        n: usize,
    }

    // Solve
    // 1, 2, 3,..., n - 1 の総和
    let ans = (n - 1) * n / 2;

    // Output
    println!("{}", ans);
}
