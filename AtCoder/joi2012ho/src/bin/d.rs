use proconio::input;

fn main() {
    // Input
    input! {
        n: usize, m: usize,
        abx_list: [(usize, usize, usize); m],
    }

    // Initialize
    // 0-basedに変更する
    let abx_list =
        abx_list
            .into_iter()
            .map(|(a, b, x)| (a - 1, b - 1, x))
            .collect::<Vec<(usize, usize, usize)>>();

    // Solve
    let sum_list: Vec<Vec<isize>> = {
        let mut sum_list: Vec<Vec<isize>> = vec![vec![0; n + 2]; n + 2];

        // +1, -1の記録
        for (a, b, x) in abx_list {
            sum_list[a][b] += 1;
            sum_list[a][b+1] -= 1;

            sum_list[a+x+2][b+1] += 1;
            sum_list[a+x+1][b] -= 1; 

            sum_list[a+x+1][b+x+2] +=1;
            sum_list[a+x+2][b+x+2] -=1;
        }

        // 左から右へのシミュレーション
        for i in 0..n + 2{
            for j in 0..n + 1 {
                sum_list[i][j + 1] += sum_list[i][j];
            }
        }

        // 左上から右下へのシミュレーション
        for i in 0..n + 1{
            for j in 0..n + 1 {
                sum_list[i + 1][j + 1] += sum_list[i][j];
            }
        }

        // 右上から左下へのシミュレーション
        for j in 0..n + 1{
            for i in 0..n + 1 {
                sum_list[i + 1][j] += sum_list[i][j];
            }
        }

        sum_list
    };


    let ans: usize = {
        let mut ans: usize = 0;
        // 一つ以上輪ゴムがかかっていればカウント
        for i in 0..n+2 {
            for j in 0..n+2 {
                if sum_list[i][j] > 0 {
                    ans += 1;
                }
            }
        }
        ans
    };

    // Output
    println!("{}", ans);
}
