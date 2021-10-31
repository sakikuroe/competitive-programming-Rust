use proconio::input;

trait Dist {
    fn dist(&self) -> usize;
}
impl Dist for (usize, usize) {
    fn dist(&self) -> usize {
        return (self.0 as isize - self.1 as isize).abs() as usize;
    }
}

fn main() {
    // Input
    input! {
        n: usize, m: usize,
        s_list: [usize; n - 1],
        a_list: [isize; m],
    }

    // Initialize
    let sum_list: Vec<usize> = {
        let mut sum_list: Vec<usize> = vec![0; n];
        for i in 0..(n - 1) {
            sum_list[i + 1] = sum_list[i] + s_list[i];
        }

        sum_list
    };

    // Solve
    let ans = {
        let mut ans = 0;
        let mut now = 0;
        for i in 0..m {
            ans += (sum_list[now], sum_list[(now as isize + a_list[i]) as usize]).dist();
            ans %= 100000;
            now = (now as isize + a_list[i]) as usize;
        }
        ans
    };

    // Output
    println!("{}", ans);
}
