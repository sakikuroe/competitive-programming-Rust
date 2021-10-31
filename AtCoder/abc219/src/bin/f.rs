
use std::collections::HashSet;

use proconio::{marker::Chars, input};

fn main() {
    // Input
    input! {
        s: Chars,
        k: isize,
    }
    
    // Initialize
    let mut map  = vec![];

    let mut now: (isize, isize) = (0, 0);
    map.push(now);
    for &c in &s{
        match c {
            'L' => now = (now.0 - 1, now.1),
            'R' => now = (now.0 + 1, now.1),
            'U' => now = (now.0 , now.1 - 1),
            'D' => now = (now.0 , now.1 + 1),
            _ => ()
        }
        map.push(now);
        
    }

    let a: HashSet<(isize, isize)> = map.clone().into_iter().collect();
    let a = a.len();

    for &c in &s{
        match c {
            'L' => now = (now.0 - 1, now.1),
            'R' => now = (now.0 + 1, now.1),
            'U' => now = (now.0 , now.1 - 1),
            'D' => now = (now.0 , now.1 + 1),
            _ => ()
        }
        map.push(now);
        
    }

    let b: HashSet<(isize, isize)> = map.clone().into_iter().collect();
    let b = b.len();

    println!("{} {}", a, b);

    println!("{}" ,a as isize + (b as isize- a as isize) * (k as isize - 1));

    
    // Solve
    
    // Output

}
