use proconio::input;

fn main() {
    // Input
    input! {
        n: usize,
    }

    // Solve
    let mut ans = 0;
    for i in 1..=n {
        let mut cnt = 0;
        for j in 1..=i {
            if i % j == 0 {
                cnt += 1;
            }
        }
        if cnt == 8 && i % 2 == 1 {
            ans += 1;
        }
    }

    // Output
    println!("{}", ans);
}
