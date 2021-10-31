use num::integer::lcm;
use proconio::input;

fn exponent_of_two(mut n: usize) -> usize {
    let mut cnt = 0;
    while n % 2 == 0 {
        n /= 2;
        cnt += 1;
    }

    cnt
}

fn solve(a_list: &mut Vec<usize>, mut m: usize) -> usize {
    let t = exponent_of_two(a_list[0]);

    for a in a_list.into_iter() {
        if exponent_of_two(*a) != t {
            return 0;
        }
    }

    for a in a_list.into_iter() {
        *a = *a >> t;
    }
    m = m >> t;

    let mut l: usize = 1;
    for a in a_list.into_iter() {
        l = lcm(l, *a);
        if l > m {
            return 0;
        }
    }
    m = m / l;
    m = (m + (2 - 1)) / 2;

    m
}

fn main() {
    // Input
    input! {
        n: usize, m: usize,
        a_list: [usize; n],
    }

    // Initialize
    let mut a_list = a_list.into_iter().map(|x| x / 2).collect::<Vec<usize>>();

    // Solve
    let ans = solve(&mut a_list, m);

    // Output
    println!("{}", ans);
}
