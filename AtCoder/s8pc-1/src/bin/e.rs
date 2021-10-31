use proconio::input;
use std::collections::VecDeque;

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

fn solve(a_list: &Vec<usize>, c_list: &VecDeque<usize>) -> usize {
    // distance_sum_list[i] := 街1から街(i + 1)までの距離
    let sum_list: Vec<usize> = {
        // distance_list[i] := 街(i + 1)と街(i + 2)を結ぶ道路の長さ
        let distance_list: Vec<usize> = {
            let mut distance_list = vec![];
            for i in 1..a_list.len() {
                distance_list.push(pow_mod(a_list[i - 1], a_list[i]));
            }
            distance_list
        };

        let mut sum_list = vec![0; a_list.len()];
        for i in 0..a_list.len() - 1 {
            sum_list[i + 1] = sum_list[i] + distance_list[i];
        }
        sum_list
    };

    let ans = {
        let mut ans: usize = 0;
        // sum_listでの累積和でクエリに対する答えを高速に計算
        for i in 0..(c_list.len() - 1) {
            ans += (sum_list[c_list[i + 1] - 1] as isize - sum_list[c_list[i] - 1] as isize).abs()
                as usize;
            ans %= MOD;
        }

        ans % MOD
    };

    ans
}

fn output(ans: usize) {
    println!("{}", ans);
}

fn main() {
    // Input
    input! {
        n: usize, q: usize,
        a_list: [usize; n],
        mut c_list: [usize; q],
    }

    // Initialize
    let mut c_list: VecDeque<usize> = c_list.into_iter().collect();
    c_list.push_front(1);
    c_list.push_back(1);

    // Solve
    let ans: usize = solve(&a_list, &c_list);

    // Output
    output(ans);
}
