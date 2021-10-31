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

fn is_prime(n: usize) -> bool {
        if n == 0 || n == 1 {
                false
        } else {
                for i in 2..=(n as f64).sqrt() as usize {
                        if n % i == 0 {
                                return false;
                        }
                }
                true
        }
}

fn prime_factorization(n: usize) -> Vec<usize> {
        let mut res: Vec<usize> = vec![];
        let mut n = n;
        let mut i = 2;

        while i <= (n as f64).sqrt() as usize {
                if n % i == 0 {
                        res.push(i);
                        n /= i;
                        i = 2;
                } else {
                        i += 1;
                }
        }

        if is_prime(n) {
                res.push(n);
        }

        res
}

fn main() {
        // Input
        input! {
                n: usize,
        }

        // Solve
        let prime_list = {
                let mut res = vec![0; 1001];
                for i in 2..=n {
                        let v = prime_factorization(i);
                        for x in v {
                                res[x] += 1;
                        }
                }

                res
        };

        let ans = {
                let mut res = Mint::new(1);
                for i in 1..=1000 {
                        res *= Mint::new(prime_list[i] + 1);
                }

                res.a
        };

        // Output
        println!("{}", ans);
}
