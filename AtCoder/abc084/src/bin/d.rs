use proconio::input;

fn is_prime(n: usize) -> bool {
    if n == 0 || n == 1 {
        return false;
    }
    for i in 2..=(n as f64).powf(0.5) as usize {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    // Input
    input! {
        q: usize,
        lr: [(usize, usize); q],
    }

    // Solve
    let s = {
        let mut res = vec![0; 100001];
        for i in 1..=100000 {
            if i % 2 == 1 && is_prime(i) && is_prime((i + 1) / 2) {
                res[i] += 1;
            }
        }
        for i in 0..100000 {
            res[i + 1] += res[i];
        }
        res
    };

    let mut ans = vec![];
    for (l, r) in lr {
        ans.push(s[r] - s[l - 1]);
    }

    // Output
    for ans in ans {
        println!("{}", ans);
    }
}
