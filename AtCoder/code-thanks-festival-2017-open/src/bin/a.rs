use proconio::input;

fn main() {
        // Input
        input! {
                mut t_list: [usize; 8],
        }

        // Solve
        let ans = t_list.into_iter().max().unwrap();

        // Output
        println!("{}", ans);
}
