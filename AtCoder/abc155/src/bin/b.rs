use proconio::input;

fn main() {
        // Input
        input! {
            n: usize,
            a_list: [usize;n],
        }

        // Initialize

        // Solve
        let ans = a_list
                .iter()
                .map(|x| x % 2 != 0 || (x % 3 == 0 || x % 5 == 0))
                .fold(true, |x, y| x & y);

        // Output
        if ans {
                println!("APPROVED");
        } else {
                println!("DENIED");
        }
}
