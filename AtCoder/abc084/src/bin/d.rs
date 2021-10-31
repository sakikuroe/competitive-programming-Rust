use proconio::input;

trait Prime {
    fn is_prime(self) -> bool;
}

impl Prime for usize {
    fn is_prime(self) -> bool {
        if self == 0 || self == 1 {
            false
        } else {
            for i in 2..=(self as f64).sqrt() as usize {
                if self % i == 0 {
                    return false;
                }
            }
            true
        }
    }
}

fn solve(lr_list: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut ans_list: Vec<usize> = vec![];

    // sum_list[i] := i以下にある2017に似た数
    let sum_list: Vec<usize> = {

        // like_2017_list[i] = iが2017に似た数かどうか
        let like_2017_list: Vec<bool> = (0..100001).map(|n| n.is_prime() && ((n + 1) / 2).is_prime()).collect();

        // 累積和を作る
        let mut sum_list: Vec<usize> = vec![0; 100001];
        for i in 1..100001 {
            if like_2017_list[i] {
                sum_list[i] = sum_list[i - 1] + 1;
            } else {
                sum_list[i] = sum_list[i - 1];
            }
        }

        sum_list
    };


    // sum_listでの累積和でクエリに対する答えを高速に計算
    for &(l, r) in lr_list {
        ans_list.push(sum_list[r] - sum_list[l - 1]);
    }

    ans_list
}

fn output(ans_list: &Vec<usize>) {
    for ans in ans_list {
        println!("{}", ans);
    }
}

fn main() {
    // Input
    input! {
        q: usize,
        lr_list: [(usize, usize); q],
    }

    // Solve
    let ans_list: Vec<usize> = solve(&lr_list);

    // Output
    output(&ans_list);
}
