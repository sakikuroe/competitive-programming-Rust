use proconio::input;
use std::ops;
const MOD: usize = 1_000_000_007;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Mint {
    value: usize,
}
impl Mint {
    pub fn new(value: usize) -> Mint {
        Mint {
            value: value % MOD,
        }
    }
    #[allow(dead_code)]
    pub fn value(&self) -> usize {
        self.value
    }
    pub fn pow(&self, n: usize) -> Mint {
        let mut res = Mint::new(1);
        let mut n = n;
        let mut x = self.clone();
        while n > 0 {
            if n & 1 == 1 {
                res *= x;
            }
            x = x * x;
            n = n >> 1;
        }

        res
    }
    pub fn inverse(&self) -> Mint {
        self.pow(MOD - 2)
    }
    pub fn factorial(&self) -> Mint {
        if self.value == 0 {
            Mint::new(1)
        } else {
            let mut res = Mint::new(1);
            for i in 1..=self.value {
                res *= Mint::new(i);
            }
            res
        }
    }
}
impl ops::Add for Mint {
    type Output = Mint;
    fn add(self, other: Self) -> Self {
        let mut value = self.value + other.value;
        if value >= MOD {
            value -= MOD;
        }
        Mint {
            value,
        }
    }
}
impl ops::Sub for Mint {
    type Output = Mint;
    fn sub(self, other: Self) -> Self {
        if self.value >= other.value {
            Mint {
                value: self.value - other.value,
            }
        } else {
            Mint {
                value: (self.value + MOD) - other.value,
            }
        }
    }
}
impl ops::Mul for Mint {
    type Output = Mint;
    fn mul(self, other: Self) -> Self {
        Mint {
            value: self.value * other.value % MOD,
        }
    }
}
impl ops::Div for Mint {
    type Output = Mint;
    fn div(self, other: Self) -> Self {
        Mint {
            value: (self * other.inverse()).value % MOD,
        }
    }
}
impl ops::AddAssign for Mint {
    fn add_assign(&mut self, other: Self) {
        self.value = (*self + other).value;
    }
}
impl ops::SubAssign for Mint {
    fn sub_assign(&mut self, other: Self) {
        self.value = (*self - other).value;
    }
}
impl ops::MulAssign for Mint {
    fn mul_assign(&mut self, other: Self) {
        self.value = (*self * other).value;
    }
}
impl ops::DivAssign for Mint {
    fn div_assign(&mut self, other: Self) {
        self.value = (*self / other).value;
    }
}

pub fn permutation(n: Mint, r: Mint) -> Mint {
    let mut res = Mint::new(1);
    for i in 0..r.value() {
        res *= Mint::new(n.value() - i);
    }
    res
}

pub fn combination(n: Mint, r: Mint) -> Mint {
    r.factorial().inverse() * permutation(n, r)
}

fn main() {
    // Input
    input! {
        w: usize,
        h: usize,
    }

    // Solve
    let ans = combination(Mint::new(w + h - 2), Mint::new(h - 1)).value();

    // Output
    println!("{}", ans);
}
