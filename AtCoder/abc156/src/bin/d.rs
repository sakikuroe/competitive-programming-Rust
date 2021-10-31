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

fn main() {
        // Input
        input! {
            n: usize, a: usize, b: usize,
        }

        // Solve
        let ans = {
                let mut res = Mint::new(2).mod_pow(n) - Mint::new(1);
                res -= mod_combination(Mint::new(n), Mint::new(a));
                res -= mod_combination(Mint::new(n), Mint::new(b));
                res.a
        };

        // Output
        println!("{}", ans);
}
