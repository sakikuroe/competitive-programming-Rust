use proconio::input;

fn main() {
    // Input
    input! {
        n: usize, m: usize,
        s: [usize; n - 1],
        a: [isize; m],
    }

    // Solve
    let sum = {
        let mut res = vec![0; n];
        for i in 0..(n - 1) {
            res[i + 1] = res[i] + s[i];
        }
        res
    };

    let mut ans = 0;

    let mut now = 0;
    for i in 0..m {
        if a[i] > 0 {
            ans += sum[(now + a[i]) as usize] - sum[now as usize];
        } else {
            ans += sum[now as usize] - sum[(now + a[i]) as usize];
        }
        now += a[i];
        ans %= 100000;
    }

    // Output
    println!("{}", ans);
}
