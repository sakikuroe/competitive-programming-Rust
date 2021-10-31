use proconio::input;

fn main() {
    // Input
    input! {
        m: usize,
        dc_list: [(usize, usize); m],
    }

    // Solve
    // 桁数
    let d = {
        let mut res = 0;
        for &(_, c) in &dc_list {
            res += c;
        }
        res
    };

    // 各桁の和
    let s = {
        let mut res = 0;
        for &(d, c) in &dc_list {
            res += d * c;
        }
        res
    };

    let ans = (d - 1) + (s - 1) / 9;

    // Output
    println!("{}", ans);
}
