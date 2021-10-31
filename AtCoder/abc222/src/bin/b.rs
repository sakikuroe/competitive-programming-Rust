use proconio::input;

fn main() {
        // Input
        input! {
                n: usize, p: usize,
                a: [usize; n],
        }

        // Initialize

        // Solve

        // Output
        println!("{}", a.into_iter().filter(|x| *x < p).collect::<Vec<usize>>().len());
}
