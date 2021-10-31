use proconio::{input, marker::Chars};

fn main() {
    // Input
    input! {
        a: Chars,
    }

    // Solve
    let ans = {
        if a == vec!['a'] {
            "-1"
        } else {
            "a"
        }
    };

    // Output
    println!("{}", ans);
}
