use memoise::memoise;
use proconio::input;

#[memoise(1 <= i <= 100, 0 <= n <= 20)]
// (i, n) -> i番目までの数字で答えがnとなるような演算子の組の数
fn dp(i: usize, n: usize, digit_list: &Vec<usize>) -> usize {
    if i == 1 {
        return if n == digit_list[i - 1] { 1 } else { 0 };
    }

    let res: usize = if digit_list[i - 1] <= n {
        dp(i - 1, n - digit_list[i - 1], digit_list)
    } else {
        0
    } + if n + digit_list[i - 1] <= 20 {
        dp(i - 1, n + digit_list[i - 1], digit_list)
    } else {
        0
    };

    res
}

fn solve(digit_list: &Vec<usize>, result: usize) -> usize {
    dp(digit_list.len(), result, digit_list)
}

fn output(ans: usize) {
    println!("{}", ans);
}

fn main() {
    // Input
    input! {
        n: usize,
        mut digit_list: [usize; n - 1], result: usize,
    }

    // Solve
    let ans = solve(&digit_list, result);

    // Output
    output(ans);
}
