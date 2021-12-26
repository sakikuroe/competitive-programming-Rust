use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;
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

fn main() {
    // Input
    input! {
        n: usize, q: usize,
        a: [usize; n],
        c: [Usize1; q],
    }

    // Solve
    let mut c: VecDeque<usize> = c.into_iter().collect();
    c.push_front(0);
    c.push_back(0);

    let s = {
        let mut res = vec![Mint::new(0); n];
        for i in 0..n - 1 {
            res[i + 1] = res[i] + Mint::new(a[i]).pow(a[i+1]);
        }
        res
    };

    let mut ans = Mint::new(0);
    for i in 0..q+1 {
        let a = std::cmp::min(c[i], c[i+1]);
        let b = std::cmp::max(c[i], c[i+1]);
        ans += s[b] - s[a];
    }
    
    // Output
    println!("{}", ans.value());
}
