use proconio::input;

fn main() {
    // Input
    input! {
        h: usize, w: usize, k: usize, v: usize,
        a: [[usize; w]; h],
    }

    // Solve
    let s = {
        let mut res = vec![vec![0; w + 1]; h + 1];
        for i in 0..h {
            for j in 0..w {
                res[i + 1][j + 1] = a[i][j] + k;
            }
        }
        for i in 0..h {
            for j in 0..w {
                res[i + 1][j + 1] += res[i][j + 1] + res[i + 1][j] - res[i][j];
            }
        }
        res
    };

    let mut ans = 0;
    for i in 0..=h {
        for j in 0..=w {
            for k in 0..i {
                for l in 0..j {
                    let cost = s[i][j] + s[k][l] - s[i][l] - s[k][j];
                    if cost <= v {
                        ans = std::cmp::max(ans, (i - k) * (j - l));
                    }
                }
            }
        }
    }

    // Output
    println!("{}", ans);
}
