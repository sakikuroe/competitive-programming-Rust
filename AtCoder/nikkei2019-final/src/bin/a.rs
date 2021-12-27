use proconio::input;

fn main() {
    // Input
    input! {
        n: usize,
        a: [usize; n],
    }

    // Solve
    let s = {
        let mut res = vec![0; n + 1];
        for i in 0..n {
            res[i + 1] = a[i];
        }
        for i in 0..n {
            res[i + 1] += res[i];
        }
        res
    };

    let mut ans = vec![];
    for k in 1..=n {
        let mut a = vec![];
        for j in 0..n - (k - 1) {
            a.push(s[j + k] - s[j]);
        }
        ans.push(a.into_iter().max())
    }

    // Output
    for ans in ans {
        println!("{}", ans.unwrap());
    }
}
