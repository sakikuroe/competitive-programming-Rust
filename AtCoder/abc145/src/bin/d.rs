use proconio::input;
use std::ops;
const MOD: usize = 1_000_000_007;
#[derive(Clone, Debug)]
struct Mint {
        a: usize,
}
impl Mint {
        fn new(a: usize) -> Mint {
                Mint {
                        a: a % MOD,
                }
        }
        fn mod_pow(&self, n: usize) -> Mint {
                if n == 0 {
                        Mint::new(1)
                } else if n % 2 == 0 {
                        Mint::new((self.clone() * self.clone()).mod_pow(n / 2).a % MOD)
                } else {
                        Mint::new((self.clone() * self.clone().mod_pow(n - 1)).a % MOD)
                }
        }
        fn mod_inverse(&self) -> Mint {
                self.mod_pow(MOD - 2)
        }
        fn mod_factorial(&self) -> Mint {
                if self.a == 0 {
                        Mint::new(1)
                } else {
                        let mut res = Mint::new(1);
                        for i in 1..=self.a {
                                res *= Mint::new(i);
                        }
                        res
                }
        }
}
impl ops::Add for Mint {
        type Output = Mint;
        fn add(self, other: Self) -> Self {
                let mut a = self.a + other.a;
                if a >= MOD {
                        a -= MOD;
                }
                Mint {
                        a,
                }
        }
}
impl ops::Sub for Mint {
        type Output = Mint;
        fn sub(self, other: Self) -> Self {
                if self.a >= other.a {
                        Mint {
                                a: self.a - other.a,
                        }
                } else {
                        Mint {
                                a: (self.a + MOD) - other.a,
                        }
                }
        }
}
impl ops::Mul for Mint {
        type Output = Mint;
        fn mul(self, other: Self) -> Self {
                Mint {
                        a: self.a * other.a % MOD,
                }
        }
}
impl ops::Div for Mint {
        type Output = Mint;
        fn div(self, other: Self) -> Self {
                Mint {
                        a: (self * other.mod_inverse()).a % MOD,
                }
        }
}
impl ops::AddAssign for Mint {
        fn add_assign(&mut self, other: Self) {
                self.a = (self.clone() + other).a;
        }
}
impl ops::SubAssign for Mint {
        fn sub_assign(&mut self, other: Self) {
                self.a = (self.clone() - other).a;
        }
}
impl ops::MulAssign for Mint {
        fn mul_assign(&mut self, other: Self) {
                self.a = (self.clone() * other).a;
        }
}
impl ops::DivAssign for Mint {
        fn div_assign(&mut self, other: Self) {
                self.a = (self.clone() / other).a;
        }
}
fn mod_permutation(n: Mint, r: Mint) -> Mint {
        let mut res = Mint::new(1);
        for i in 0..r.a {
                res *= Mint::new(n.a - i);
        }
        res
}
fn mod_combination(n: Mint, r: Mint) -> Mint {
        r.mod_factorial().mod_inverse() * mod_permutation(n, r)
}

// a(2, 1) + b(1, 2) = (x, y)となるような(a, b)の組を返す
fn calc_coefficient(x: usize, y: usize) -> (usize, usize) {
        let a: usize = (2 * x - y) / 3;
        let b: usize = (2 * y - x) / 3;

        (a, b)
}

fn solve(x: usize, y: usize) -> usize {
        if (x + y) % 3 != 0 || y > 2 * x || x > 2 * y {
                0
        } else {
                let (a, b) = calc_coefficient(x, y);
                mod_combination(Mint::new(a + b), Mint::new(b)).a
        }
}

fn main() {
        // Input
        input! {
            x: usize,
            y: usize,
        }

        // Solve
        let ans = solve(x, y);

        // Output
        println!("{}", ans);
}
