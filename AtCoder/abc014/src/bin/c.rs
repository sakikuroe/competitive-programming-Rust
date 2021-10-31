use proconio::input;

fn main() {
    // Input
    input! {
        n: usize,
        ab_list: [(usize, usize); n],
    }

    // Initialize
    let sum_list: Vec<isize> = {
        // いもす法を行う
        let mut sum_list: Vec<isize> = vec![0; 1_000_000 + 2];
        for &(a, b) in &ab_list {
            sum_list[a] += 1;
            sum_list[b + 1] -= 1;
        }

        for i in 0..1_000_001 {
            sum_list[i + 1] += sum_list[i];
        }

        sum_list
    };

    // Solve
    let ans: usize = sum_list.into_iter().max().unwrap() as usize;

    // Output
    println!("{}", ans);
}
