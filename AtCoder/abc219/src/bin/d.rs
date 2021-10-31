use proconio::{marker::Chars, input};
use memoise::memoise;
use std::usize::MAX;
use std::cmp;
const INF: usize = MAX;

trait Op {
    fn extended_add(&self) -> usize;
}
impl Op for (usize, usize) {
    fn extended_add(&self) -> usize {
        if let Some(value) = self.0.checked_add(self.1) {
            return value;
        } else {
            return INF;
        }
    }
}

#[memoise(0<= n <= 330, 0<=x<=330, 0<=y<=330)]
fn dp(n: usize, x: isize, y: isize, ab_list:&Vec<(isize, isize)>) -> usize {
    if n == 0 {
        return INF
    }
    if n == 1 {
        let (a, b) = ab_list[0];
        if a >= x && b >= y {
            return 1;
        } else {
            return INF;
        }
    }
    if x == 0 && y == 0 {
        return 0;
    }

    let (a, b) = ab_list[n - 1];
    if a < x || b < y {
        return cmp::min((dp(n - 1, cmp::max(x- a, 0), cmp::max(y- b, 0), ab_list) , 1).extended_add(), dp(n - 1, x,  y, ab_list));
    } else {
        return 1;
    }

    INF

}

fn main() {
    // Input
    input! {
        n: usize,
        x: isize, y:isize,
        ab_list :[(isize, isize); n],
    }
    
    // Initialize

    let ans = dp(n, x, y,&ab_list);

    // for i in 0..=3 {
    //     for j in 0..=x {
    //         for k in 0..=y {
    //             println!("{} {} {} {}", i,j,k,dp(i,j,k,&ab_list));
    //         }
    //     }
    // }

    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }

    
    // Solve
    
    // Output

}
