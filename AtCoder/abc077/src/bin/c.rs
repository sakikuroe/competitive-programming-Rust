use proconio::input;
use superslice::*;

fn main() {
    // Input
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
        mut c: [usize; n],
    }

    // Solve
    a.sort();
    b.sort();
    c.sort();

    let mut ans = 0;
    for i in 0..n {
        ans += a.lower_bound(&b[i]) * (n - c.upper_bound(&b[i]));
    }

    // Output
    println!("{}", ans);
}
