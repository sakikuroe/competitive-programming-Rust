use cargo_snippet::snippet;

#[snippet(name = "mod")]
const MOD: usize = 1_000_000_007;
#[snippet(name = "mod")]
fn mod_pow(n: usize, m: usize) -> usize {
        if m == 0 {
                1
        } else if m % 2 == 0 {
                mod_pow(n * n % MOD, m / 2) % MOD
        } else {
                n * mod_pow(n, m - 1) % MOD
        }
}
#[snippet(name = "mod")]
fn mod_factorial(n: usize) -> usize {
        if n == 0 {
                1
        } else {
                n * mod_factorial(n - 1) % MOD
        }
}
#[snippet(name = "mod")]
fn mod_combination(n: usize, r: usize) -> usize {
        mod_factorial(n) * mod_inverse(mod_factorial(r)) % MOD * mod_inverse(mod_factorial(n - r))
                % MOD
}
#[snippet(name = "mod")]
fn mod_inverse(n: usize) -> usize {
        mod_pow(n, MOD - 2)
}

fn main() {
        println!("Hello, world!");
}
