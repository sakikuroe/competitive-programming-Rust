use memoise::memoise;
use proconio::input;
use std::cmp;

#[memoise (0 <= left <= 3000, 0 <= right <= 3000)]
fn dp(left: usize, right: usize, a_list: &Vec<isize>) -> isize {
        if right - left == 1 {
                return a_list[left];
        }

        return cmp::max(
                a_list[left] - dp(left + 1, right, a_list),
                a_list[right - 1] - dp(left, right - 1, a_list),
        );
}

fn main() {
        // Input
        input! {
                n: usize,
                a_list: [isize; n],
        }

        // Solve
        let ans = dp(0, n, &a_list);

        // Output
        println!("{}", ans);
}
