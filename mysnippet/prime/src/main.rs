use cargo_snippet::snippet;

#[snippet(name = "prime")]
trait Prime {
        fn is_prime(self) -> bool;
}
#[snippet(name = "prime")]
impl Prime for usize {
        fn is_prime(self) -> bool {
                if self == 0 || self == 1 {
                        false
                } else {
                        for i in 2..=(self as f64).sqrt() as usize {
                                if self % i == 0 {
                                        return false;
                                }
                        }
                        true
                }
        }
}

#[snippet(name = "dist")]
trait Dist {
        fn dist(&self) -> usize;
}
#[snippet(name = "dist")]
impl Dist for (usize, usize) {
        fn dist(&self) -> usize {
                return (self.0 as isize - self.1 as isize).abs() as usize;
        }
}

fn main() {
        println!("Hello, world!");
}
