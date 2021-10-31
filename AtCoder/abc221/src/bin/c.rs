use proconio::input;
use proconio::marker::Chars;

fn main() {
        // Input
        input! {
                mut n: Chars,
        }

        // Initialize
        n.sort();
        n.reverse();

        // Solve
        let mut ans = 0;
        for bit in 0..(1 << n.len()) {
                let mut a: Vec<char> = vec![];
                let mut b: Vec<char> = vec![];
                for i in 0..n.len() {
                        if (bit >> i) & 1 == 1 {
                                a.push(n[i]);
                        } else {
                                b.push(n[i]);
                        }
                }

                if a.len() == 0 || b.len() == 0 {
                        continue;
                }

                let a = a.into_iter().collect::<String>().parse::<usize>().unwrap();
                let b = b.into_iter().collect::<String>().parse::<usize>().unwrap();

                ans = std::cmp::max(ans, a * b);
        }

        println!("{}", ans);
        // Output
}
