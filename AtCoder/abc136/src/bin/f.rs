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

fn compress(v: &mut Vec<isize>) -> Vec<isize> {
        let mut vs = vec![];
        for a in v.into_iter() {
                vs.push((*a) as isize);
        }
        vs.sort();
        vs.dedup();
        for a in v.into_iter() {
                *a = vs.binary_search(a).unwrap() as isize;
        }
        vs
}

fn main() {
        // Input
        input! {
                n: usize,
                mut xy_list: [(isize, isize); n],
        }

        // Initialize
        xy_list.sort_by_key(|x| x.0);
        let mut y_list = vec![];
        for i in 0..n {
                y_list.push(xy_list[i].1);
        }

        compress(&mut y_list);
        let y_list = y_list.iter().map(|&y| (y + 1) as usize).collect::<Vec<usize>>();

        // Solve
        let ans = {
                let mut res = Mint::new(0);
                let mut bit1 = Bit::new(200000);
                let mut bit2 = {
                        let mut res = Bit::new(200000);
                        for &y in &y_list {
                                res.add(y as usize, 1);
                        }
                        res
                };

                for i in 0..n {
                        bit2.add(y_list[i] as usize, -1);

                        let c = bit1.sum(y_list[i]) as usize;
                        let a = bit1.sum(200000) as usize - c;
                        let d = bit2.sum(y_list[i]) as usize;
                        let b = bit2.sum(200000) as usize - d;

                        // if p_{i} \in T
                        res += Mint::new(2).mod_pow(n - 1);
                        // if p_{i} \notin T
                        res += (Mint::new(2).mod_pow(a) - Mint::new(1))
                                * (Mint::new(2).mod_pow(d) - Mint::new(1))
                                * Mint::new(2).mod_pow(b + c);
                        res += (Mint::new(2).mod_pow(b) - Mint::new(1))
                                * (Mint::new(2).mod_pow(c) - Mint::new(1))
                                * Mint::new(2).mod_pow(a + d);
                        res -= (Mint::new(2).mod_pow(a) - Mint::new(1))
                                * (Mint::new(2).mod_pow(b) - Mint::new(1))
                                * (Mint::new(2).mod_pow(c) - Mint::new(1))
                                * (Mint::new(2).mod_pow(d) - Mint::new(1));

                        bit1.add(y_list[i] as usize, 1);
                }

                res.a
        };

        // Output
        println!("{}", ans);
}
