use std::io;
// use proconio::input;

const MOD: usize = 1_000_000_007;

fn pow_mod(n: usize, m: usize) -> usize {
    if m == 0 {
        1
    } else if m % 2 == 0 {
        pow_mod(n * n % MOD, m / 2)
    } else {
        n * pow_mod(n, m - 1) % MOD
    }
}

fn solve(n: usize, m: usize) -> usize {
    pow_mod(n, m)
}

fn output(ans: usize) {
    println!("{}", ans);
}

fn main() {
    // Input
        // input! {
        //     n: usize, m: usize,
        // }
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let word_list: Vec<&str> = line.split_whitespace().collect();
    let n: usize = word_list[0].parse::<usize>().unwrap();
    let m: usize = word_list[1].parse::<usize>().unwrap();

    // Solve
    let ans: usize = solve(n, m);

    // Output
    output(ans);
}
