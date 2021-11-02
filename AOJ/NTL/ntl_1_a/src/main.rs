use std::io;
// use proconio::input;

trait Prime {
    fn is_prime(self) -> bool;
}

impl Prime for usize {
    fn is_prime(self) -> bool {
        if self == 1 {
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
fn solve(mut n: usize) -> Vec<usize> {
    let mut res: Vec<usize> = vec![];

    for i in 2..=(n as f64).sqrt() as usize {
        while n % i == 0 {
            res.push(i);
            n /= i;
        }
    }
    
    if n != 1 {
        res.push(n);
    }

    res
}

fn output(n: usize, primes: &Vec<usize>) {
    print!("{}:", n);
    for x in primes {
        print!(" {}", x);
    }
    println!();
}

fn main() {
    // Input
        // input! {
        //     n: usize,
        // }
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let word_list: Vec<&str> = line.split_whitespace().collect();
    let n: usize = word_list[0].parse::<usize>().unwrap();

    // Solve
    let primes: Vec<usize> = solve(n);

    // Output
    output(n, &primes);
}
