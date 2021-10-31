use proconio::input;
use std::ops;
const MOD: usize = 998244353;
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

fn compress(v: &mut Vec<usize>) -> Vec<usize> {
        let mut vs = vec![];
        for a in v.into_iter() {
                vs.push(((*a) as isize) as usize);
        }
        vs.sort();
        vs.dedup();
        for a in v.into_iter() {
                *a = vs.binary_search(a).unwrap();
        }
        vs
}

#[derive(Debug)]
struct Bit {
        size: usize,
        bit: Vec<isize>,
}
impl Bit {
        fn new(n: usize) -> Bit {
                Bit {
                        size: n,
                        bit: vec![0; n + 1],
                }
        }
        fn sum(&self, mut idx: usize) -> isize {
                let mut res = 0;
                while idx > 0 {
                        res += self.bit[idx];
                        idx -= (idx as isize & -(idx as isize)) as usize;
                }
                res
        }
        fn add(&mut self, mut idx: usize, a: isize) {
                while idx <= self.size {
                        self.bit[idx] = self.bit[idx] + a;
                        idx += (idx as isize & -(idx as isize)) as usize;
                }
        }
}

fn main() {
        // Input
        input! {
                n: usize,
                mut x_list: [usize; n],
        }

        // Initialize
        compress(&mut x_list);
        let x_list = x_list.into_iter().map(|x| x + 1).collect::<Vec<usize>>();

        // Solve
        let mut bit = Bit::new(300_300);
        let ans = {
                let mut res = Mint::new(0);
                for (i, &x) in x_list.iter().enumerate() {
                        res += Mint::new(bit.sum(x) as usize) * Mint::new(2).mod_pow(i);
                        bit.add(x, (Mint::new(2).mod_inverse().mod_pow(i + 1)).a as isize);
                }

                res.a
        };

        // Output
        println!("{}", ans);
}
