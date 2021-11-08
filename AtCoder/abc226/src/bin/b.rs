use proconio::input;

fn main() {
        // Input
        input! {
            n: usize,
        }

        // Solve
        let mut a: Vec<Vec<usize>> = vec![];

        for _ in 0..n {
                input! {
                    l: usize,
                    b: [usize; l],
                }
                a.push(b);
        }

        a.sort();
        a.dedup();
        let ans = a.len();

        // Output
        println!("{}", ans);
}
