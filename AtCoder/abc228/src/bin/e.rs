use proconio::input;
const MOD: usize = 998244353;
fn mod_pow(a: usize, n: usize, m: usize) -> usize {
        if a % m == 0 {
                0
        } else if n == 0 {
                1
        } else if n % 2 == 0 {
                mod_pow(a * a % m, n / 2, m) % m
        } else {
                a * mod_pow(a, n - 1, m) % m
        }
}
fn main() {
        // Input
        input! {
            n: usize, k: usize, m: usize,
        }

        // Solve
        let r = mod_pow(k % (MOD - 1), n, MOD - 1);
        let ans = mod_pow(m % MOD, r, MOD);

        // Output
        println!("{}", ans);
}
