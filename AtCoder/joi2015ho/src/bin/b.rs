use memoise::memoise;
use proconio::input;
use std::cmp;

#[memoise(0 <= left <= 4000, 0<= right <= 4000)]
// dp(left, right) := [cake_{left}, cake_{right})のピースからJOIくんが取れるケーキの大きさの最大値
fn dp(left: usize, right: usize, cake_list: &Vec<usize>) -> usize {
    if right - left == 1 {
        return cake_list[left];
    }
    if right - left == 2 {
        return cmp::max(cake_list[left], cake_list[left + 1]);
    }

    let res: usize = {
        let mut res = 0;
        // cake_{left}を取る場合
        // IOIちゃんが取ったケーキを元に大きさを計算
        res = cmp::max(
            res,
            cake_list[left]
                + if cake_list[left + 1] > cake_list[right - 1] {
                    dp(left + 2, right, cake_list)
                } else {
                    dp(left + 1, right - 1, cake_list)
                },
        );

        // cake_{right - 1}を取る場合
        // IOIちゃんが取ったケーキを元に大きさを計算
        res = cmp::max(
            res,
            cake_list[right - 1]
                + if cake_list[left] > cake_list[right - 2] {
                    dp(left + 1, right - 1, cake_list)
                } else {
                    dp(left, right - 2, cake_list)
                },
        );

        res
    };

    res
}

fn solve(n: usize, cake_list: &Vec<usize>) -> usize {
    let res = {
        let mut res = 0;
        // 長さ2nの列からn個の列を切り出し、最大値をとる。
        for i in 0..n {
            res = cmp::max(res, dp(i, i + n, cake_list));
        }

        res
    };

    res
}

fn output(ans: usize) {
    println!("{}", ans);
}

fn main() {
    // Input
    input! {
        n: usize,
        a_list: [usize; n],
    }

    // Initialize
    // ケーキの列を2つつ繋げた長さ2nの列を新たに作り、そこからn個の列を切り出す
    let cake_list = {
        let mut cake_list = vec![];
        for a in &a_list {
            cake_list.push(*a);
        }
        for a in &a_list {
            cake_list.push(*a);
        }

        cake_list
    };

    // Solve
    let ans: usize = solve(n, &cake_list);

    // Output
    output(ans);
}
