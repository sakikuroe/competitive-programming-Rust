use proconio::input;
use proconio::marker::Chars;

fn main() {
        // Input
        input! {
                mut s: Chars,
                mut t: Chars,
        }

        // Solve
        let mut f: bool = false;
        f = f | (s == t);
        for i in 0..s.len() - 1 {
                s.swap(i, i + 1);
                f = f | (s == t);
                s.swap(i, i + 1);
        }

        // Output
        if f {
                println!("Yes");
        } else {
                println!("No");
        }
}
